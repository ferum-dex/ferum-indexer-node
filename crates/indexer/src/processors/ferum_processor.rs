// Copyright © Aptos Foundation

use crate::{
    database::{
        clean_data_for_db, execute_with_better_error, get_chunks, PgDbPool, PgPoolConnection,
    },
    indexer::{
        errors::TransactionProcessingError, processing_result::ProcessingResult,
        transaction_processor::TransactionProcessor,
    },
    models::{
        events::EventModel,
    },
    schema,
};

use aptos_api_types::{Transaction, UserTransaction, MoveType, Address};
use async_trait::async_trait;

use diesel::{pg::upsert::excluded, result::Error, ExpressionMethods, PgConnection};
use field_count::FieldCount;
use std::{fmt::Debug};
use std::fs;

use std::borrow::Borrow;
use itertools::Itertools;

fn insert_to_db_impl(
    conn: &mut PgConnection,
    events: &[EventModel],
) -> Result<(), diesel::result::Error> {
    insert_events(conn, events)?;
    Ok(())
}

fn insert_to_db(
    conn: &mut PgPoolConnection,
    name: &'static str,
    start_version: u64,
    end_version: u64,
    events: Vec<EventModel>,
) -> Result<(), diesel::result::Error> {
    aptos_logger::trace!(
        name = name,
        start_version = start_version,
        end_version = end_version,
        "Inserting to db",
    );
    match conn
        .build_transaction()
        .read_write()
        .run::<_, Error, _>(|pg_conn| {
            insert_to_db_impl(
                pg_conn,
                &events,
            )
        }) {
        Ok(_) => Ok(()),
        Err(_) => {
            let events = clean_data_for_db(events, true);

            conn.build_transaction()
                .read_write()
                .run::<_, Error, _>(|pg_conn| {
                    insert_to_db_impl(
                        pg_conn,
                        &events,
                    )
                })
        },
    }
}

fn insert_events(
    conn: &mut PgConnection,
    items_to_insert: &[EventModel],
) -> Result<(), diesel::result::Error> {
    use schema::events::dsl::*;
    let chunks = get_chunks(items_to_insert.len(), EventModel::field_count());
    for (start_ind, end_ind) in chunks {
        execute_with_better_error(
            conn,
            diesel::insert_into(schema::events::table)
                .values(&items_to_insert[start_ind..end_ind])
                .on_conflict((account_address, creation_number, sequence_number))
                .do_update()
                .set((
                    inserted_at.eq(excluded(inserted_at)),
                    event_index.eq(excluded(event_index)),
                )),
            None,
        )?;
    }
    Ok(())
}

pub const NAME: &str = "ferum_processor";
pub struct FerumTransactionProcessor {
    connection_pool: PgDbPool,
    ferum_addresses: Vec<Address>,
}

impl FerumTransactionProcessor {
    pub fn new(
        connection_pool: PgDbPool,
    ) -> Self {
        let ferum_config_json = fs::read_to_string("./crates/indexer/src/config/ferum.json").expect("Unable to read file");
        let addresses:Vec<String> =  serde_json::from_str(&ferum_config_json).expect("JSON was not well-formatted");
        let mut ferum_addresses:Vec<Address> = vec![];

        for address in &addresses {
            ferum_addresses.push((address.as_str()).parse::<Address>().unwrap());
        }

        Self {
            connection_pool,
            ferum_addresses,
        }
    }

    fn get_ferum_events_from_transaction(&self,
        transaction: &UserTransaction,
        transaction_block_height: i64
    ) -> Vec<EventModel> {
        let version = transaction.info.version.0 as i64;

        return transaction.events.iter()
            .enumerate()
            .filter(|(_, event)| {
                let typ = event.typ.clone();
                match typ {
                    MoveType::Struct(tag) => self.ferum_addresses.contains(&tag.address),
                    _ => false,
                }
            })
            .map(|(index, event)| EventModel::from_event(&event, version, transaction_block_height, index as i64)
            )
            .collect_vec();
    }
}

impl Debug for FerumTransactionProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = &self.connection_pool.state();
        write!(
            f,
            "FerumTransactionProcessor {{ connections: {:?}  idle_connections: {:?} }}",
            state.connections, state.idle_connections
        )
    }
}

#[async_trait]
impl TransactionProcessor for FerumTransactionProcessor {
    fn name(&self) -> &'static str {
        NAME
    }

    async fn process_transactions(
        &self,
        transactions: Vec<Transaction>,
        start_version: u64,
        end_version: u64,
    ) -> Result<ProcessingResult, TransactionProcessingError> {
        let mut conn = self.get_conn();
        let mut all_events:Vec<EventModel> = vec![];

        for transaction in &transactions {
            let block_height = transaction
                .transaction_info()
                .unwrap()
                .block_height
                .unwrap()
                .0 as i64;
            match transaction {
                Transaction::UserTransaction(user_transaction) => {
                    let mut transaction_events:Vec<EventModel> = self.get_ferum_events_from_transaction(user_transaction.borrow(), block_height);
                    all_events.append(&mut transaction_events);
                },
                _ => {},
            };
        }

        let tx_result = insert_to_db(
            &mut conn,
            self.name(),
            start_version,
            end_version,
            all_events
        );
        match tx_result {
            Ok(_) => Ok(ProcessingResult::new(
                self.name(),
                start_version,
                end_version,
            )),
            Err(err) => Err(TransactionProcessingError::TransactionCommitError((
                anyhow::Error::from(err),
                start_version,
                end_version,
                self.name(),
            ))),
        }
    }

    fn connection_pool(&self) -> &PgDbPool {
        &self.connection_pool
    }
}
