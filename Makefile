# Display Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version              # Rust compiler
	cargo --version              # Rust package manager
	rustfmt --version            # Rust code formatter
	rustup --version             # Rust toolchain manager
	clippy-driver --version      # Rust linter

# Format code using rustfmt
format:
	cargo fmt --quiet

# Run clippy for linting
lint:
	cargo clippy --quiet

# Run tests
test:
	cargo test

# Build and run the project
run:
	cargo run

# Build release version
release:
	cargo build --release

# Install Rust toolchain if needed
install:
	# Install if needed
	@echo "Updating rust toolchain"
	rustup update stable
	rustup default stable 

#all: format lint test run
times:
	cargo build --bin times && ./target/debug/times > rust_output.txt
	python3 lib/times.py  > python_output.txt

run_cli:
	cargo run -- etl
	cargo run -- create
	cargo run -- read
	cargo run -- update
	cargo run -- delete
	cargo run -- query "SELECT * FROM Universities WHERE country='United States'"
	
all: install format test lint release run_cli