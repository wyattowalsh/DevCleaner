use log::{error, warn};
use crate::config::Config;

pub fn handle_error(config: &Config, error_message: &str) {
    match config.error_handling.on_error.as_str() {
        "prompt" => {
            println!("Error: {}. Do you want to continue? (y/N)", error_message);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if !input.trim().eq_ignore_ascii_case("y") {
                std::process::exit(1);
            }
        }
        "continue" => {
            warn!("Error: {}. Continuing as per configuration.", error_message);
        }
        "abort" => {
            error!("Error: {}. Aborting as per configuration.", error_message);
            std::process::exit(1);
        }
        _ => {
            error!("Unknown error handling policy: {}. Aborting.", config.error_handling.on_error);
            std::process::exit(1);
        }
    }
}
