use chrono::Local;
use colored::*;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub fn initialize_logging() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}: {}",
                Local::now().format("%Y-%m-%d %H:%M:%S").to_string().green(),
                record.level().to_string().yellow(),
                record.target().to_string().cyan(),
                record.args().to_string().white()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}
