use crate::config::lattice_config::CONFIG;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::{config::Config, NoTls};

pub fn initialize_pool(size: usize) -> Pool {
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
    Pool::builder(mgr).max_size(size).build().unwrap()
}

pub fn print_config() {
    println!("Server Address: {}", CONFIG.listen_socket_addr);
    println!("Max Connections: {}", CONFIG.max_connections);
    println!("Postgres Address: {}", CONFIG.pg_host);
    println!("Postgres Username: {}", CONFIG.pg_user);
    println!("Postgres Password: {}", CONFIG.pg_pass);
    println!("Postgres Database: {}", CONFIG.pg_db_name);
}
