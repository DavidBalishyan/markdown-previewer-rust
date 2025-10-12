# Cargo run
default:
	cargo run

# Build the project in release mode
build:
    cargo build --release

# Run the project from source
run:
	cargo run

# Lint with Clippy
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Format the code
fmt:
    cargo fmt --all

# Clean build artifacts
clean:
    cargo clean
