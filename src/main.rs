mod lattice_config;
use lattice_config::CONFIG;

fn main() {
    println!("Server Address: {}", CONFIG.server_address);
    println!("Max Connections: {}", CONFIG.max_connections);
    println!("Port: {}", CONFIG.port);
}
