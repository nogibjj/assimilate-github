# Makefile for Rust project

# Variables
TARGET = target/release/my_project
SRC = $(wildcard src/*.rs)

# Default target: build the project
.PHONY: all
all: $(TARGET)

# Build the release version
$(TARGET): $(SRC)
	cargo build --release

# Clean build files
.PHONY: clean
clean:
	cargo clean

# Run tests
.PHONY: test
test:
	cargo test

# Run the binary
.PHONY: run
run: $(TARGET)
	./$(TARGET)

# Format the source code
.PHONY: format
format:
	cargo fmt

# Lint the source code
.PHONY: lint
lint:
	cargo clippy

# Update dependencies
.PHONY: update
update:
	cargo update
