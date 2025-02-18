use log::{info, warn, error, LevelFilter};
use simplelog::{Config as LogConfig, WriteLogger, TermLogger, TerminalMode, ColorChoice};
use std::fs::File;
use std::path::Path;
use crate::config::Config;

pub fn init_logger(config: &Config) {
    let log_level = match config.logging.console_output.as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let term_logger = TermLogger::new(
        log_level,
        LogConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    let file_logger = WriteLogger::new(
        log_level,
        LogConfig::default(),
        File::create(Path::new(&config.logging.file_path)).unwrap(),
    );

    if let Err(e) = simplelog::CombinedLogger::init(vec![term_logger, file_logger]) {
        eprintln!("Failed to initialize logger: {}", e);
    }
}

pub fn log_progress(message: &str) {
    info!("{}", message);
}

pub fn log_summary(message: &str) {
    info!("{}", message);
}

pub fn log_error(message: &str) {
    error!("{}", message);
}
