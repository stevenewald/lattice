use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;
use std::net::{SocketAddr, ToSocketAddrs};

pub struct LatticeConfig {
    pub listen_socket_addr: SocketAddr,
    pub sql_addr: String,
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
        let sql_addr = match env::var("DATABASE_URL") {
            Ok(addr) => addr,
            Err(_) => panic!("Database URL not set"),
        };

        let listen_socket_addr = listen_addr
            .to_socket_addrs()
            .expect("Failed to parse socket IP")
            .next()
            .expect("No socket addresses found");

        LatticeConfig {
            listen_socket_addr,
            sql_addr,
            max_connections,
        }
    };
}
