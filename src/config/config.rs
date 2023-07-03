use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;
use std::net::{SocketAddr, ToSocketAddrs};

pub struct LatticeConfig {
    pub listen_socket_addr: SocketAddr,
    pub pg_host: String,
    pub pg_port: u16,
    pub pg_db_name: String,
    pub pg_pass: String,
    pub pg_user: String,
    pub max_connections: u32,
}

lazy_static! {
    pub static ref CONFIG: LatticeConfig = {
        dotenv().ok();

        let listen_addr = match env::var("LISTEN_URL") {
            Ok(addr) => addr,
            Err(_) => "127.0.0.1:3000".to_string(),
        };
        let max_connections = match env::var("MAX_CONNECTIONS") {
            Ok(maxconn) => match maxconn.parse::<u32>() {
                Ok(maxconn) => maxconn as u32,
                Err(_) => 50,
            },
            Err(_) => 50,
        };
        let pg_host = match env::var("PG_HOST") {
            Ok(addr) => addr,
            Err(_) => panic!("Postgres host not set"),
        };
        let pg_db_name = match env::var("PG_DB_NAME") {
            Ok(db_name) => db_name,
            Err(_) => panic!("Postgres db name not set"),
        };
        let pg_pass = match env::var("PG_PASS") {
            Ok(pass) => pass,
            Err(_) => panic!("Postgres password not set"),
        };
        let pg_user = match env::var("PG_USER") {
            Ok(user) => user,
            Err(_) => panic!("Postgres username not set"),
        };
        let pg_port: u16 = match env::var("PG_PORT") {
            Ok(port) => port.parse().expect("Unable to parse port"),
            Err(_) => panic!("Postgres port not set"),
        };

        let listen_socket_addr = listen_addr
            .to_socket_addrs()
            .expect("Failed to parse socket IP")
            .next()
            .expect("No socket addresses found");

        LatticeConfig {
            listen_socket_addr,
            pg_host,
            pg_port,
            pg_db_name,
            pg_pass,
            pg_user,
            max_connections,
        }
    };
}
