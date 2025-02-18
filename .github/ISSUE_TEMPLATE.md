# Issue Template

## Description

<!-- Please provide a clear and concise description of the issue. -->

## Steps to Reproduce

<!-- Please provide the steps to reproduce the issue. -->

1. Step 1
2. Step 2
3. Step 3

## Expected Behavior

<!-- Please describe the expected behavior. -->

## Actual Behavior

<!-- Please describe the actual behavior. -->

## Configuration

<!-- If applicable, provide the configuration file or relevant settings. -->

```toml
# Example configuration
dependency_directories = ["node_modules", ".venv"]
logging = { console_output = "color", file_path = "logs/dep_cleaner.log", structured_format = "json" }
error_handling = { on_error = "prompt" }
additional = { default_target_directories = ["/path/to/project1", "/path/to/project2"], symlink_handling = "detect" }
```

## Environment

<!-- Please provide information about your environment. -->

- OS: [e.g., Windows, macOS, Linux]
- Rust version: [e.g., 1.56.0]
- dep_cleaner version: [e.g., 1.0]

> [!TIP]
> Please include logs, relevant configuration, and steps to reproduce for faster resolution.

## Additional Context

<!-- Add any other context about the problem here. -->
