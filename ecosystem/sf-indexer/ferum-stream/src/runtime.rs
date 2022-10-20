// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::metrics;

use aptos_api::context::Context;
use aptos_api_types::{AsConverter, Transaction, UserTransaction, MoveType, IdentifierWrapper, Address, MoveStructTag, EventGuid};
use aptos_config::config::NodeConfig;
use aptos_logger::{debug, error, warn};
use aptos_types::chain_id::ChainId;
use aptos_vm::data_cache::StorageAdapterOwned;
use futures::channel::mpsc::channel;
use prost::Message;
use std::{convert::TryInto, sync::Arc, time::Duration};
use std::borrow::Borrow;
use std::str::FromStr;
use aptos_protos::ferum::v1::{
    OrderCreateEvent as FerumOrderCreateEvent,
    OrderFinalizeEvent as FerumOrderFinalizeEvent,
    OrderExecutionEvent as FerumOrderExecutionEvent,
    PriceUpdateEvent as FerumPriceUpdateEvent,
};
use storage_interface::state_view::DbStateView;
use tokio::runtime::{Builder, Runtime};
use tokio::time::sleep;
use itertools::Itertools;
use storage_interface::DbReader;
use crate::convert::convert_ferum_event;

#[derive(Debug)]
pub enum FerumEvent {
    OrderCreate(FerumOrderCreateEvent),
    OrderFinalize(FerumOrderFinalizeEvent),
    OrderExecution(FerumOrderExecutionEvent),
    PriceUpdate(FerumPriceUpdateEvent),
}
impl FerumEvent {
    fn encode(&self, buf: &mut Vec<u8>) -> Result<(), prost::EncodeError>
    {
        match self {
            Self::OrderCreate(e) => e.encode(buf),
            Self::OrderFinalize(e) => e.encode(buf),
            Self::OrderExecution(e) => e.encode(buf),
            Self::PriceUpdate(e) => e.encode(buf),
        }
    }
}

/// Creates a runtime which creates a thread pool which pushes firehose of block protobuf to SF endpoint
/// Returns corresponding Tokio runtime
pub fn bootstrap(
    config: &NodeConfig,
    chain_id: ChainId,
    db: Arc<dyn DbReader>,
) -> Option<anyhow::Result<Runtime>> {
    if config.ferum.address.is_none() {
        return None;
    }

    let runtime = Builder::new_multi_thread()
        .thread_name("fh-stream")
        .enable_all()
        .build()
        .expect("[fh-stream] failed to create runtime");

    let node_config = config.clone();

    runtime.spawn(async move {
        let (fake_mp_sender, _) = channel(1);
        let context = Context::new(chain_id, db, fake_mp_sender, node_config.clone());
        let context_arc = Arc::new(context);
        let starting_block = node_config.ferum.starting_block.unwrap_or(0);
        let mut streamer = FirehoseStreamer::new(
            context_arc,
            starting_block,
            Address::from(node_config.ferum.address.clone().unwrap()),
        );
        streamer.start().await;
    });
    Some(Ok(runtime))
}

pub struct FirehoseStreamer {
    pub context: Arc<Context>,
    pub resolver: Arc<StorageAdapterOwned<DbStateView>>,
    pub current_block_height: u64,
    pub current_epoch: u64,

    // So we don't need to recompute each single time.
    ferum_address: Address,
    ferum_module_identifier: IdentifierWrapper,
    ferum_create_event_identifier: IdentifierWrapper,
    ferum_finalize_event_identifier: IdentifierWrapper,
    ferum_execution_event_identifier: IdentifierWrapper,
    ferum_price_event_identifier: IdentifierWrapper,
}

impl FirehoseStreamer {
    pub fn new(
        context: Arc<Context>,
        starting_block: u64,
        ferum_address: Address,
    ) -> Self {
        let resolver = Arc::new(context.move_resolver().unwrap());
        let (_block_start_version, _block_last_version, block_event) = context
            .db
            .get_block_info_by_height(starting_block)
            .unwrap_or_else(|_| {
                panic!(
                    "Could not get block_info for starting block {}",
                    starting_block,
                )
            });

        Self {
            context,
            resolver,
            current_block_height: block_event.height(),
            current_epoch: block_event.epoch(),

            ferum_address,
            ferum_module_identifier: IdentifierWrapper::from_str(&"market").unwrap(),
            ferum_create_event_identifier: IdentifierWrapper::from_str(&"CreateEvent").unwrap(),
            ferum_finalize_event_identifier: IdentifierWrapper::from_str(&"FinalizeEvent").unwrap(),
            ferum_execution_event_identifier: IdentifierWrapper::from_str(&"ExecutionEvent").unwrap(),
            ferum_price_event_identifier: IdentifierWrapper::from_str(&"PriceUpdateEvent").unwrap(),
        }
    }

    pub async fn start(&mut self) {
        // Format is FIRE INIT aptos-node <PACKAGE_VERSION> <MAJOR_VERSION> <MINOR_VERSION> <CHAIN_ID>
        println!(
            "\nFIRE INIT aptos-node {} aptos 0 0 {}",
            env!("CARGO_PKG_VERSION"),
            self.context.chain_id().id(),
        );
        loop {
            self.convert_next_block().await;
        }
    }

    pub async fn convert_next_block(&mut self) -> Vec<FerumEvent> {
        let mut result: Vec<FerumEvent> = vec![];


        let (block_start_version, block_last_version, _) = match self
            .context
            .db
            .get_block_info_by_height(self.current_block_height)
        {
            Ok(block_info) => block_info,
            Err(err) => {
                // TODO(ferum): handle block being pruned.
                warn!(
                    "[fh-stream] failed to get block info for block_height={}. Error: {}",
                    self.current_block_height, err
                );

                // Thought that this would fix the issue with the db not having all the data when
                // launching as a secondary instance but it didn't.
                // // Try to catch up with the primary.
                // match self.context.db.try_catch_up_with_primary() {
                //     Ok(_) => {},
                //     _ => warn!("[fh-stream] unable to catch up with primary"),
                // };

                sleep(Duration::from_millis(300)).await;
                return vec![];
            }
        };

        let ledger_info = self.context.get_latest_ledger_info_wrapped().unwrap();
        let block_timestamp = self
            .context
            .db
            .get_block_timestamp(block_start_version)
            .unwrap_or_else(|_| {
                panic!(
                    "Could not get timestamp for version {}",
                    block_start_version
                )
            });
        // We are validating the block as we convert and print each transactions. The rules are as follows:
        // 1. first (and only first) transaction is a block metadata or genesis 2. versions are monotonically increasing 3. start and end versions match block boundaries
        // Retry if the block is not valid. Panic if there's anything wrong with encoding a transaction.
        println!("\nFIRE BLOCK_START {}", self.current_block_height);

        let transactions = match self.context.get_transactions(
            block_start_version,
            (block_last_version - block_start_version + 1)
                .try_into()
                .unwrap(),
            ledger_info.version(),
        ) {
            Ok(transactions) => transactions,
            Err(err) => {
                error!("[fh-stream] failed to get transactions: {}", err);
                sleep(Duration::from_millis(100)).await;
                return vec![];
            }
        };

        if transactions.is_empty() {
            debug!("[fh-stream] no transactions to send");
            sleep(Duration::from_millis(100)).await;
            return vec![];
        }
        debug!(
            "[fh-stream] got {} transactions from {} to {} [version on last actual transaction {}]",
            transactions.len(),
            block_start_version,
            block_last_version,
            transactions.last().map(|txn| txn.version).unwrap_or(0)
        );

        let mut curr_version = block_start_version;
        for onchain_txn in transactions {
            let txn_version = onchain_txn.version;
            let mut txn: Option<Transaction> = None;
            let mut retries = 0;
            while txn.is_none() {
                match self
                    .resolver
                    .as_converter(self.context.db.clone())
                    .try_into_onchain_transaction(block_timestamp, onchain_txn.clone())
                {
                    Ok(transaction) => {
                        txn = Some(transaction);
                    }
                    Err(err) => {
                        if retries == 0 {
                            aptos_logger::debug!(
                                "Could not convert onchain transaction, trying again with updated resolver",
                            );
                        } else {
                            panic!("Could not convert onchain transaction, error: {:?}", err);
                        }
                        retries += 1;
                        self.resolver = Arc::new(self.context.move_resolver().unwrap());
                    }
                }
            }
            let txn = txn.unwrap();
            if !self.validate_transaction_type(curr_version == block_start_version, &txn) {
                error!(
                    "Block {} failed validation: first transaction has to be block metadata or genesis",
                    self.current_block_height,
                );
                sleep(Duration::from_millis(500)).await;
                return vec![];
            }
            if curr_version != txn_version {
                error!(
                    "Block {} failed validation: missing version {}",
                    self.current_block_height, curr_version,
                );
                sleep(Duration::from_millis(500)).await;
                return vec![];
            }

            curr_version += 1;

            match txn {
                Transaction::UserTransaction(user_txn) => {
                    let mut ferum_events = self.get_ferum_events_from_transaction(user_txn.borrow());
                    self.print_ferum_events(self.current_block_height, &ferum_events);
                    result.append(&mut ferum_events);
                },
                _ => {},
            };

            metrics::TRANSACTIONS_PROCESSED.inc();
        }

        if curr_version - 1 != block_last_version {
            error!(
                "Block {} failed validation: last version supposed to be {} but getting {}",
                self.current_block_height,
                block_last_version,
                curr_version - 1,
            );
            sleep(Duration::from_millis(500)).await;
            return vec![];
        }

        println!("\nFIRE BLOCK_END {}", self.current_block_height);
        metrics::BLOCKS_PROCESSED.inc();
        self.current_block_height += 1;
        result
    }

    /// First, and only first, transaction in a block has to be bmt or genesis
    fn validate_transaction_type(&self, is_first_txn: bool, transaction: &Transaction) -> bool {
        let is_bm_or_genesis = matches!(
            transaction,
            Transaction::BlockMetadataTransaction(_) | Transaction::GenesisTransaction(_)
        );
        is_first_txn == is_bm_or_genesis
    }

    fn print_ferum_events(&self, block: u64, events: &Vec<FerumEvent>) {
        events.iter().for_each(|e| {
            let mut buf = vec![];
            e.encode(&mut buf).unwrap_or_else(|_| {
                panic!("Could not convert protobuf event to bytes '{:?}'", e)
            });
            let typ = match e {
                FerumEvent::OrderCreate(_) => "CREATE",
                FerumEvent::OrderExecution(_) => "EXEC",
                FerumEvent::OrderFinalize(_) => "FIN",
                FerumEvent::PriceUpdate(_) => "PUPD",
            };
            println!("\nFIRE EVENT {} {} {}", block, typ, base64::encode(buf));
            metrics::FERUM_EVENTS_SENT.inc();
        })
    }

    fn get_ferum_events_from_transaction(&self, transaction: &UserTransaction) -> Vec<FerumEvent> {
        transaction.events.iter()
            .filter_map(|e| {
                let key = e.guid;
                let data = e.data.clone();
                let typ = e.typ.clone();
                match typ {
                    MoveType::Struct(tag) => self.parse_ferum_event(&key, &tag, data),
                    _ => None,
                }
            })
            .filter_map(|e| convert_ferum_event(&e).ok())
            .collect_vec()
    }

    fn parse_ferum_event(&self, key: &EventGuid, tag: &MoveStructTag, data: serde_json::Value) -> Option<FerumEvent> {
        if !tag.address.eq(&self.ferum_address) || !tag.module.eq(&self.ferum_module_identifier) {
            return None
        }

        // TOOD: need to convert to a string first because there is a bug with trying to parse a serde Value directly.
        // Bug stems from u64 ints being treated as numbers and strings.

        if tag.name.eq(&self.ferum_create_event_identifier) {
            match serde_json::from_str::<FerumOrderCreateEvent>(data.clone().to_string().as_str()) {
                Ok(event) => Some(FerumEvent::OrderCreate(event)),
                Err(err) => {
                    error!("Unable to parse ferum order create event. key: {:?}. data: {}. err: {:?}", key, data, err);
                    None
                }
            }
        } else if tag.name.eq(&self.ferum_finalize_event_identifier) {
            match serde_json::from_str::<FerumOrderFinalizeEvent>(data.clone().to_string().as_str()) {
                Ok(event) => Some(FerumEvent::OrderFinalize(event)),
                Err(err) => {
                    error!("Unable to parse ferum order finalize event. key: {:?}. data: {}. err: {:?}", key, data, err);
                    None
                }
            }
        } else if tag.name.eq(&self.ferum_execution_event_identifier) {
            match serde_json::from_str::<FerumOrderExecutionEvent>(data.clone().to_string().as_str()) {
                Ok(event) => Some(FerumEvent::OrderExecution(event)),
                Err(err) => {
                    error!("Unable to parse ferum order execution event. key: {:?}. data: {}. err: {:?}", key, data, err);
                    None
                }
            }
        } else if tag.name.eq(&self.ferum_price_event_identifier) {
            match serde_json::from_str::<FerumPriceUpdateEvent>(data.clone().to_string().as_str()) {
                Ok(event) => Some(FerumEvent::PriceUpdate(event)),
                Err(err) => {
                    error!("Unable to parse ferum price update event. key: {:?}. data: {}. err: {:?}", key, data, err);
                    None
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use aptos_protos::ferum::v1::{OrderCreateEvent, PriceUpdateEvent, TypeInfo};
    use crate::convert::{convert_type_info, convert_hex_string, convert_ferum_event};
    use crate::runtime::FerumEvent;

    #[test]
    fn test_convert_hex() {
        assert_eq!("test_coins", convert_hex_string("0x746573745f636f696e73".to_string()).unwrap());
        assert!(convert_hex_string("0x1".to_string()).is_err());
        assert!(convert_hex_string("1".to_string()).is_err());
        assert_eq!("test_coins", convert_hex_string("746573745f636f696e73".to_string()).unwrap());
        assert_eq!("t", convert_hex_string("0x74".to_string()).unwrap());
        assert_eq!("t", convert_hex_string("74".to_string()).unwrap());
        assert_eq!("", convert_hex_string("".to_string()).unwrap());
        assert_eq!("", convert_hex_string("0x".to_string()).unwrap());
    }

    #[test]
    fn test_convert_type_info() {
        let info = TypeInfo {
            account_address: "0x33a6417b3846f094782adb6de9c0f6256c70502bf85e483d53140be0ff87a6fd".to_string(),
            module_name: "0x746573745f636f696e73".to_string(),
            struct_name: "0x46616b654d6f6e657942".to_string(),
        };

        let expected_info = TypeInfo {
            account_address: "0x33a6417b3846f094782adb6de9c0f6256c70502bf85e483d53140be0ff87a6fd".to_string(),
            module_name: "test_coins".to_string(),
            struct_name: "FakeMoneyB".to_string(),
        };

        assert_eq!(expected_info, convert_type_info(&info).unwrap());
    }

    #[test]
    fn test_deserialize_create_event() {
        let data = r#"
            {
              "orderID": {
                "counter": "0",
                "owner": "0x33a6417b3846f094782adb6de9c0f6256c70502bf85e483d53140be0ff87a6fd"
              },
              "orderMetadata": {
                "clientOrderID": "",
                "instrumentType": {
                  "account_address": "0x33a6417b3846f094782adb6de9c0f6256c70502bf85e483d53140be0ff87a6fd",
                  "module_name": "0x746573745f636f696e73",
                  "struct_name": "0x46616b654d6f6e657941"
                },
                "originalQty": {
                  "val": "10000000000"
                },
                "price": {
                  "val": "20000000000"
                },
                "quoteType": {
                  "account_address": "0x33a6417b3846f094782adb6de9c0f6256c70502bf85e483d53140be0ff87a6fd",
                  "module_name": "0x746573745f636f696e73",
                  "struct_name": "0x46616b654d6f6e657942"
                },
                "remainingQty": {
                  "val": "10000000000"
                },
                "side": 1,
                "status": 1,
                "type": 2
              },
              "timestampMicroSeconds": "1662710248328332"
            }
        "#;

        let evnt: OrderCreateEvent = serde_json::from_str(data).unwrap();
        let out = convert_ferum_event(&FerumEvent::OrderCreate(evnt)).unwrap();
        let converted_create_evt = match out {
           FerumEvent::OrderCreate(evt) => Some(evt),
           _ => None,
        }.unwrap();
        assert_eq!(
            "test_coins",
            converted_create_evt.
                order_metadata.unwrap().
                instrument_type.unwrap().
                module_name,
        );

        return
    }

    #[test]
    fn test_deserialize_price_update_event() {
        let data = r#"
            {
              "data": {
                "askSize": { "val": "0" },
                "bidSize": { "val": "0" },
                "instrumentType": {
                  "account_address": "0x5cb3ebb4b0f67b6240a324aa88161672729d3b11fc1227d73d109d94a10d159a",
                  "module_name": "0x746573745f636f696e73",
                  "struct_name": "0x46616b654d6f6e657941"
                },
                "maxBid": { "val": "0" },
                "minAsk": { "val": "0" },
                "quoteType": {
                  "account_address": "0x5cb3ebb4b0f67b6240a324aa88161672729d3b11fc1227d73d109d94a10d159a",
                  "module_name": "0x746573745f636f696e73",
                  "struct_name": "0x46616b654d6f6e657942"
                },
                "timestampMicroSeconds": "1663119658851306"
              }
            }
        "#;

        let evnt: PriceUpdateEvent = serde_json::from_str(data).unwrap();
        let out = convert_ferum_event(&FerumEvent::PriceUpdate(evnt)).unwrap();
        let converted_create_evt = match out {
            FerumEvent::OrderCreate(evt) => Some(evt),
            _ => None,
        }.unwrap();
        assert_eq!(
            "test_coins",
            converted_create_evt.
                order_metadata.unwrap().
                instrument_type.unwrap().
                module_name,
        );

        return
    }
}
