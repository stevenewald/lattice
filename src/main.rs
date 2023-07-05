use crate::config::{initialize, lattice_config::CONFIG};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Error, Server};
use networking::handlers::request_handler;
use piping::piping::create_pipe;

mod caching;
mod config;
mod networking;
mod piping;
mod query_parsing;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let pool = initialize::initialize_pool(16);

    //TODO: make non-hardcoded buf size. Maybe dynamic?
    let (col_update_sender, col_updete_receiver) = create_pipe();

    let make_service = make_service_fn(|_conn| {
        let pool = pool.clone();
        let sender = col_update_sender.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                let pool = pool.clone();
                let sender = sender.clone();
                request_handler(req, pool, sender)
            }))
        }
    });

    initialize::print_config();

    Server::bind(&CONFIG.listen_socket_addr)
        .serve(make_service)
        .await?;
    Ok(())
}
