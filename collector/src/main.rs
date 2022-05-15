use common::{err, metrics, storage, tracing, Result};
use hyperlocal::SocketIncoming;
use tokio::{select, sync::mpsc};

use crate::{config::USER_AGENT, segment::Segment};

mod api;
mod config;
mod emitter;
mod error;
mod event;
mod ingester;
mod namespace;
mod producer;
mod segment;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::init();

    let args: config::Args = argh::from_env();

    let (ingest_sender, ingest_receiver) = mpsc::channel::<Segment>(args.ingest_queue_depth);
    let (flush_sender, flush_receiver) = mpsc::channel::<Segment>(args.flush_queue_depth);
    let mut registry = metrics::Registry::new(config::BINARY);
    let bucket = storage::Observed::new(
        &mut registry,
        storage::Gcs::new(args.bucket.clone(), args.service_account, USER_AGENT)?,
    );

    let api = api::Api::new(&mut registry);
    let api_handle = tokio::spawn(api.bind(SocketIncoming::bind(args.api_path)?, ingest_sender));
    tokio::pin!(api_handle);

    let ingester = ingester::Observed::new(
        &mut registry,
        ingester::InMemory::new(
            args.segment_byte_limit,
            args.segment_count_limit,
            args.flush_interval,
        ),
    );
    let ingester_handle = tokio::spawn(ingester::run(
        args.flush_interval,
        ingester,
        ingest_receiver,
        flush_sender,
    ));
    tokio::pin!(ingester_handle);

    let emitter_handle = tokio::spawn(emitter::run(
        emitter::Observed::new(
            &mut registry,
            args.bucket,
            emitter::Gcs::new(
                config::BINARY.to_string(),
                common::host().ok_or_else(|| err!("failed to get hostname"))?,
                bucket,
            ),
        ),
        flush_receiver,
    ));
    tokio::pin!(emitter_handle);

    let metrics_server_handle =
        tokio::spawn(metrics::bind(args.metrics_addr, config::BINARY, registry));
    tokio::pin!(metrics_server_handle);

    loop {
        select! {
            join = &mut api_handle => join??,
            join = &mut ingester_handle => join??,
            join = &mut metrics_server_handle => join??,
            join = &mut emitter_handle => join??,

            else => break,
        }
    }

    Ok(())
}
