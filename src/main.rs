use crate::config::{initialize, lattice_config::CONFIG};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Error, Server};

mod caching;
mod config;
mod networking;
mod piping;
mod query_parsing;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let pool = initialize::initialize_pool(16);

    let make_service = make_service_fn(|_conn| {
        let pool = pool.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                let pool = pool.clone();
                networking::handlers::request_handler(req, pool)
            }))
        }
    });

    initialize::print_config();

    Server::bind(&CONFIG.listen_socket_addr)
        .serve(make_service)
        .await?;
    Ok(())
}
