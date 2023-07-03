use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Error, Server};
use lattice_config::CONFIG;
use tokio_postgres::{config::Config, NoTls};

mod networking;
mod lattice_config;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let mut pg_config = Config::new();
    pg_config.user(&CONFIG.pg_user);
    pg_config.password(&CONFIG.pg_pass);
    pg_config.host(&CONFIG.pg_host);
    pg_config.dbname(&CONFIG.pg_db_name);
    pg_config.port(CONFIG.pg_port);

    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool: Pool = Pool::builder(mgr).max_size(16).build().unwrap();

    let make_service = make_service_fn(|_conn| {
        let pool = pool.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                let pool = pool.clone();
                networking::handlers::request_handler(req, pool)
            }))
        }
    });

    println!("Server Address: {}", CONFIG.listen_socket_addr);
    println!("Max Connections: {}", CONFIG.max_connections);
    println!("Postgres Address: {}", CONFIG.pg_host);
    println!("Postgres Username: {}", CONFIG.pg_user);
    println!("Postgres Password: {}", CONFIG.pg_pass);
    println!("Postgres Database: {}", CONFIG.pg_db_name);

    Server::bind(&CONFIG.listen_socket_addr)
        .serve(make_service)
        .await?;
    Ok(())
}
