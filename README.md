# dep_cleaner

Rust CLI to carefully remove project dependencies (node_modules/, .venv/, etc ...) from a rig

## Overview

dep_cleaner is a production-ready CLI tool that recursively scans specified directories (and their subdirectories) to locate and delete common dependency directories (e.g., node_modules, .venv, etc.). Its behavior is fully configurable via an external configuration file.

## Features

- Accepts one or more target directories as CLI arguments.
- Supports standard flags:
  - --help for usage information.
  - --version to display version details.
  - --config <path> to specify a custom configuration file.
  - --dry_run flag to simulate deletions by reporting the directories that would be removed.
- Recursively traverses provided directories to locate dependency directories as specified in the configuration.
- Handles symbolic links with options to follow them, ignore them, or detect and avoid circular references.
- Fully configurable via a configuration file in formats such as config.toml or config.yaml.
- Optimized directory scanning using multi-threading or asynchronous I/O.
- Robust error handling and logging system.
- Graceful shutdown handling for OS signals (e.g., SIGINT/SIGTERM).

## Usage

### Basic Usage

```sh
dep_cleaner <directories> [OPTIONS]
```

### Examples

```sh
# Scan and delete dependency directories in the specified directories
dep_cleaner /path/to/project1 /path/to/project2

# Use a custom configuration file
dep_cleaner /path/to/project --config /path/to/config.toml

# Simulate deletions without performing them
dep_cleaner /path/to/project --dry_run
```

## Configuration

dep_cleaner supports configuration files in formats such as config.toml or config.yaml. Below is an example of a config.toml file with detailed comments explaining each configuration option.

```toml
# Configuration file for dep_cleaner

# List of dependency directories to search for and delete
# Example: ["node_modules", ".venv"]
dependency_directories = ["node_modules", ".venv"]

# Logging settings
[logging]
# Console output settings
# Options: "plain", "color"
console_output = "color"

# File-based logging settings
# Path to the log file
file_path = "logs/dep_cleaner.log"

# Structured logging settings
# Options: "json", "plain"
structured_format = "json"

# Error handling policies
[error_handling]
# Options: "prompt", "continue", "abort"
on_error = "prompt"

# Additional parameters
[additional]
# Default target directories to scan
# Example: ["/path/to/project1", "/path/to/project2"]
default_target_directories = ["/path/to/project1", "/path/to/project2"]

# Symlink handling preferences
# Options: "follow", "ignore", "detect"
symlink_handling = "detect"
```

## Build and Installation

### Prerequisites

- Rust (latest stable version)

### Building the Project

```sh
cargo build --release
```

### Running Tests

```sh
cargo test
```

## Contribution Guidelines

We welcome contributions to dep_cleaner! Please follow these guidelines when contributing:

1. Fork the repository and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. Ensure the test suite passes.
4. Make sure your code lints.
5. Create a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
