# Makefile for Rust project
# Format source code using rustfmt
format:
	cargo fmt --all

# Run Clippy for linting with warnings treated as errors
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Run the test suite
test:
	cargo test --all

# Build the crate in release mode
build:
	cargo build --release

# Allow "make all" to run everything
all: format lint test build

.PHONY: format lint test build all
