// Copyright © Aptos Foundation

// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_metrics_core::{register_int_counter, IntCounter};
use once_cell::sync::Lazy;

pub static TRANSACTIONS_PROCESSED: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "aptos_ferum_transactions_processed_count",
        "Blocks processed by Ferum indexer",
    )
    .unwrap()
});

pub static FERUM_EVENTS_SENT: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "aptos_ferum_events_sent_count",
        "Ferum events converted and printed out to stdout, picked up by the StreamingFast Firehose indexer",
    )
    .unwrap()
});

pub static BLOCKS_PROCESSED: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(
        "aptos_ferum_blocks_processed_count",
        "Blocks processed by Ferum indexer",
    )
    .unwrap()
});
