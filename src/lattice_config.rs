use config::Config;
use lazy_static::lazy_static;

pub struct AppConfig {
    pub server_address: String,
    pub max_connections: u32,
    pub port: u16,
}

lazy_static! {
    pub static ref CONFIG: AppConfig = {
        let settings = Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .add_source(config::Environment::with_prefix("LATTICE"))
            .build()
            .unwrap();

        let server_address_result = settings.get_string("server_address");
        let max_connections_result = settings.get_int("max_connections");
        let port_result = settings.get_int("port");

        let server_address: String = match server_address_result {
            Ok(value) => value,
            Err(_) => "".to_string(),
        };
        let max_connections: u32 = match max_connections_result {
            Ok(value) => {
                if value >= 0 {
                    value as u32
                } else {
                    0
                }
            }
            Err(_) => 0,
        };
        let port: u16 = match port_result {
            Ok(value) => {
                if value >= 0 {
                    value as u16
                } else {
                    0
                }
            }
            Err(_) => 0,
        };

        AppConfig {
            server_address,
            max_connections,
            port,
        }
    };
}
