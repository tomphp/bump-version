SOURCE_FILES := $(shell find src -type f)

target/debug/versioned-files: Cargo.toml Cargo.lock src/ $(SOURCE_FILES)
	cargo build

target/release/versioned-files: Cargo.toml Cargo.lock src/ $(SOURCE_FILES)
	cargo build --release

.PHONY=clean
clean:
	rm -rf target/

.PHONY=check
lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings -D clippy::all -D clippy::pedantic -D clippy::cargo -A clippy::multiple-crate-versions
	cargo check

.PHONY=format
format:
	cargo fix --allow-dirty --allow-staged
	cargo fmt --all

.PHONY=test-integration
test-integration: target/release/versioned-files
	cargo test --test '*' --locked

.PHONY=test-unit
test-unit:
	cargo test --bins --locked

.PHONY=test
test: test-unit test-integration

.PHONY=precommit
precommit: lint test
