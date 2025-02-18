# Makefile for dep_cleaner

# Variables
CARGO := cargo
TARGET := target/release/dep_cleaner

# Default target
all: build

# Build the project
build:
	$(CARGO) build --release

# Run tests
test:
	$(CARGO) test

# Lint the code
lint:
	$(CARGO) clippy -- -D warnings

# Format the code
format:
	$(CARGO) fmt

# Clean the project
clean:
	$(CARGO) clean

# Run the project
run: build
	$(TARGET) $(ARGS)

.PHONY: all build test lint format clean run
