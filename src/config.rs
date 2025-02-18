use serde::Deserialize;
use std::fs;
use std::path::Path;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub dependency_directories: Vec<String>,
    pub logging: LoggingConfig,
    pub error_handling: ErrorHandlingConfig,
    pub additional: AdditionalConfig,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub console_output: String,
    pub file_path: String,
    pub structured_format: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorHandlingConfig {
    pub on_error: String,
}

#[derive(Debug, Deserialize)]
pub struct AdditionalConfig {
    pub default_target_directories: Vec<String>,
    pub symlink_handling: String,
}

pub fn load_config(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

impl Default for Config {
    fn default() -> Self {
        Config {
            dependency_directories: vec!["node_modules".to_string(), ".venv".to_string()],
            logging: LoggingConfig {
                console_output: "color".to_string(),
                file_path: "logs/dep_cleaner.log".to_string(),
                structured_format: "json".to_string(),
            },
            error_handling: ErrorHandlingConfig {
                on_error: "prompt".to_string(),
            },
            additional: AdditionalConfig {
                default_target_directories: vec!["/path/to/project1".to_string(), "/path/to/project2".to_string()],
                symlink_handling: "detect".to_string(),
            },
        }
    }
}
