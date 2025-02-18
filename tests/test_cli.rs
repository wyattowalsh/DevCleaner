use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("dep_cleaner").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("dep_cleaner").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn test_dry_run_flag() {
    let temp_dir = tempdir().unwrap();
    let node_modules_dir = temp_dir.path().join("node_modules");
    fs::create_dir(&node_modules_dir).unwrap();

    let mut cmd = Command::cargo_bin("dep_cleaner").unwrap();
    cmd.arg(temp_dir.path())
        .arg("--dry_run")
        .assert()
        .success()
        .stdout(predicate::str::contains("Would delete directory"));
}

#[test]
fn test_delete_confirmation() {
    let temp_dir = tempdir().unwrap();
    let node_modules_dir = temp_dir.path().join("node_modules");
    fs::create_dir(&node_modules_dir).unwrap();

    let mut cmd = Command::cargo_bin("dep_cleaner").unwrap();
    cmd.arg(temp_dir.path())
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Deleted directory"));
}

#[test]
fn test_custom_config() {
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.toml");
    fs::write(&config_path, r#"
        dependency_directories = ["node_modules"]
        [logging]
        console_output = "plain"
        file_path = "logs/dep_cleaner.log"
        structured_format = "json"
        [error_handling]
        on_error = "prompt"
        [additional]
        default_target_directories = ["/path/to/project1", "/path/to/project2"]
        symlink_handling = "detect"
    "#).unwrap();

    let node_modules_dir = temp_dir.path().join("node_modules");
    fs::create_dir(&node_modules_dir).unwrap();

    let mut cmd = Command::cargo_bin("dep_cleaner").unwrap();
    cmd.arg(temp_dir.path())
        .arg("--config")
        .arg(config_path)
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Deleted directory"));
}
