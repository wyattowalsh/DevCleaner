use clap::{App, Arg};
use std::fs;
use std::path::Path;
use std::process;

mod config;
mod logger;
mod error_handling;

fn main() {
    let matches = App::new("dep_cleaner")
        .version("1.0")
        .author("Wyatt Walsh")
        .about("CLI tool to clean up dependency directories")
        .arg(
            Arg::with_name("directories")
                .multiple(true)
                .required(true)
                .help("Target directories to scan"),
        )
        .arg(
            Arg::with_name("config")
                .long("config")
                .takes_value(true)
                .help("Path to custom configuration file"),
        )
        .arg(
            Arg::with_name("dry_run")
                .long("dry_run")
                .help("Simulate deletions without performing them"),
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("config.toml");
    let config = config::load_config(config_path).unwrap_or_else(|err| {
        eprintln!("Error loading configuration: {}", err);
        process::exit(1);
    });

    let directories: Vec<&str> = matches.values_of("directories").unwrap().collect();
    let dry_run = matches.is_present("dry_run");

    for dir in directories {
        if let Err(e) = process_directory(dir, &config, dry_run) {
            eprintln!("Error processing directory {}: {}", dir, e);
        }
    }
}

fn process_directory(dir: &str, config: &config::Config, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(dir);
    if !path.exists() {
        return Err(format!("Directory {} does not exist", dir).into());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            if config.dependency_directories.contains(&dir_name.to_string()) {
                if dry_run {
                    println!("Would delete directory: {}", path.display());
                } else {
                    println!("Are you sure you want to delete {}? (y/N)", path.display());
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input)?;
                    if input.trim().eq_ignore_ascii_case("y") {
                        fs::remove_dir_all(&path)?;
                        println!("Deleted directory: {}", path.display());
                    } else {
                        println!("Skipped directory: {}", path.display());
                    }
                }
            } else {
                process_directory(path.to_str().unwrap(), config, dry_run)?;
            }
        }
    }

    Ok(())
}
