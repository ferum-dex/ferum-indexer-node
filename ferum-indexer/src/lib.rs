// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

mod log_build_information;

use anyhow::anyhow;
use aptos_config::config::NodeConfig;
use aptos_fh_stream::runtime::bootstrap as bootstrap_fh_stream;
use aptos_logger::telemetry_log_writer::TelemetryLog;
use aptos_logger::{prelude::*};
use aptos_state_view::account_with_state_view::AsAccountWithStateView;
use aptos_types::{
    account_config::CORE_CODE_ADDRESS, account_view::AccountView, chain_id::ChainId,
};
use aptosdb::AptosDB;
use clap::Parser;
use futures::channel::mpsc;
use log_build_information::log_build_information;
use std::{
    boxed::Box,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Instant,
};
use storage_interface::{state_view::LatestDbStateCheckpointView, DbReaderWriter};
use tokio::runtime::Runtime;

const TELEMETRY_LOG_INGEST_BUFFER_SIZE: usize = 128;

/// Runs an aptos fullnode or validator
#[derive(Clone, Debug, Parser)]
#[clap(name = "Aptos Node", author, version)]
pub struct FerumIndexerConfig {
    /// Path to node configuration file (or template for local test mode)
    #[clap(short = 'f', long, parse(from_os_str), required_unless = "test")]
    config: Option<PathBuf>,
}

impl FerumIndexerConfig {
    pub fn run(self) {
        // Get the config file path
        let config_path = self.config.expect("Config is required to launch node");
        if !config_path.exists() {
            panic!(
                "The node config file could not be found! Ensure the given path is correct: {:?}",
                config_path.display()
            )
        }

        // A config file exists, attempt to parse the config
        let config = NodeConfig::load(config_path.clone()).unwrap_or_else(|error| {
            panic!(
                "Failed to parse node config file! Given file path: {:?}. Error: {:?}",
                config_path.display(),
                error
            )
        });

        // Start the node
        println!("Using node config {:?}", &config);
        start(config, None).expect("Node should start correctly");
    }
}

/// Runtime handle to ensure that all inner runtimes stay in scope
pub struct AptosHandle {
    _fh_stream: Option<Runtime>,
}

/// Start an aptos node
pub fn start(config: NodeConfig, log_file: Option<PathBuf>) -> anyhow::Result<()> {
    crash_handler::setup_panic_handler();

    let mut logger = aptos_logger::Logger::new();
    logger
        .channel_size(config.logger.chan_size)
        .is_async(config.logger.is_async)
        .level(config.logger.level)
        .telemetry_level(config.logger.telemetry_level)
        .enable_telemetry_flush(config.logger.enable_telemetry_flush)
        .console_port(config.logger.console_port)
        .read_env();
    if config.logger.enable_backtrace {
        logger.enable_backtrace();
    }
    if let Some(log_file) = log_file {
        logger.printer(Box::new(FileWriter::new(log_file)));
    }
    let mut remote_log_rx = None;
    if config.logger.enable_telemetry_remote_log {
        let (tx, rx) = mpsc::channel(TELEMETRY_LOG_INGEST_BUFFER_SIZE);
        logger.remote_log_tx(tx);
        remote_log_rx = Some(rx);
    }
    let _logger = logger.build();

    // Print out build information.
    log_build_information();

    // Let's now log some important information, since the logger is set up
    info!(config = config, "Loaded AptosNode config");

    if fail::has_failpoints() {
        warn!("Failpoints is enabled");
        if let Some(failpoints) = &config.failpoints {
            for (point, actions) in failpoints {
                fail::cfg(point, actions).expect("fail to set actions for failpoint");
            }
        }
    } else if config.failpoints.is_some() {
        warn!("failpoints is set in config, but the binary doesn't compile with this feature");
    }

    let _node_handle = setup_environment(config, remote_log_rx)?;

    let term = Arc::new(AtomicBool::new(false));

    while !term.load(Ordering::Acquire) {
        std::thread::park();
    }
    Ok(())
}

// Fetch chain ID from on-chain resource
fn fetch_chain_id(db: &DbReaderWriter) -> anyhow::Result<ChainId> {
    let db_state_view = db
        .reader
        .latest_state_checkpoint_view()
        .map_err(|err| anyhow!("[aptos-node] failed to create db state view {}", err))?;
    Ok(db_state_view
        .as_account_with_state_view(&CORE_CODE_ADDRESS)
        .get_chain_id_resource()
        .map_err(|err| anyhow!("[aptos-node] failed to get chain id resource {}", err))?
        .expect("[aptos-node] missing chain ID resource")
        .chain_id())
}

pub fn setup_environment(
    node_config: NodeConfig,
    _: Option<mpsc::Receiver<TelemetryLog>>,
) -> anyhow::Result<AptosHandle> {

    // Open the database
    let instant = Instant::now();
    let (aptos_db, db_rw) = DbReaderWriter::wrap(
        AptosDB::open_as_secondary(
            &node_config.storage.dir(),
            &node_config.storage.secondary_dir(),
            node_config.storage.rocksdb_configs,
        )
        .map_err(|err| anyhow!("DB failed to open {}", err))?,
    );
    debug!(
        "Storage service started in {} ms",
        instant.elapsed().as_millis()
    );

    let chain_id = fetch_chain_id(&db_rw)?;

    let sf_runtime = match bootstrap_fh_stream(&node_config, chain_id, aptos_db) {
        None => None,
        Some(res) => Some(res?),
    };

    Ok(AptosHandle {
        _fh_stream: sf_runtime,
    })
}
