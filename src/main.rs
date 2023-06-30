use hyper::service::{make_service_fn, service_fn};
use hyper::{Error, Server};
use lattice_config::CONFIG;
use sqlx::postgres::PgPool;

mod lattice_config;
mod services;
mod handlers;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>>{
    let pool = PgPool::connect(&CONFIG.sql_addr)
        .await
        .expect("Could not initialize DB CONN");

    let make_service = make_service_fn(move |_| {
        let pool = pool.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                handlers::example_handler(req, pool.clone())
            }))
        }
    });

    println!("Server Address: {}", CONFIG.listen_socket_addr);
    println!("SQL Address: {}", CONFIG.sql_addr);
    println!("Max Connections: {}", CONFIG.max_connections);

    Server::bind(&CONFIG.listen_socket_addr).serve(make_service).await?;
    Ok(())
}
