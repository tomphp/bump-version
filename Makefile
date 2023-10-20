SOURCE_FILES := $(shell find src -type f)

target/debug/bump-version: Cargo.toml Cargo.lock src/ $(SOURCE_FILES)
	cargo build

target/release/bump-version: Cargo.toml Cargo.lock src/ $(SOURCE_FILES)
	cargo build --release

.PHONY=clean
clean:
	rm -rf target/

.PHONY=check
check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings -D clippy::all -D clippy::pedantic -D clippy::cargo -A clippy::multiple-crate-versions
	cargo check

.PHONY=format
format:
	cargo fix --allow-dirty --allow-staged
	cargo fmt --all

.PHONY=test-integration
test-integration: target/release/bump-version
	cargo test --test '*'