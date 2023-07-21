use crate::caching::caching_data::CachingData;
use crate::config::logging::initialize_logging;
use crate::config::{initialize, lattice_config::CONFIG};
use caching::cache_loop::start_cache_rx;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Error, Server};
use networking::handlers::request_handler;
use piping::piping::create_pipe;
use std::sync::Arc;
use tokio::sync::RwLock;

mod caching;
mod config;
mod networking;
mod piping;
mod query_parsing;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    initialize_logging();

    let pool = initialize::initialize_pool(16);

    //TODO: make non-hardcoded buf size. Maybe dynamic?
    let (col_update_sender, col_update_receiver) = create_pipe();
    let caching_lock: Arc<RwLock<CachingData>> = start_cache_rx(col_update_receiver);
    let make_service = make_service_fn(|_conn| {
        let pool = pool.clone();
        let sender = col_update_sender.clone();
        let caching_info = Arc::clone(&caching_lock);
        async {
            Ok::<_, Error>(service_fn(move |req| {
                let pool = pool.clone();
                let sender = sender.clone();
                let caching_info = caching_info.clone();
                request_handler(req, pool, sender, caching_info)
            }))
        }
    });

    initialize::print_config();

    Server::bind(&CONFIG.listen_socket_addr)
        .serve(make_service)
        .await?;
    Ok(())
}
