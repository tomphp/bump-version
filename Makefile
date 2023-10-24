SOURCE_FILES := $(shell find src -type f)

ifeq ($(OS),Windows_NT)
	TARGET_EXTENSION := .exe
else
	TARGET_EXTENSION :=
endif

TARGET := versioned-files$(TARGET_EXTENSION)

target/debug/$(TARGET): Cargo.toml Cargo.lock src/ $(SOURCE_FILES)
	cargo build

target/release/$(TARGET): Cargo.toml Cargo.lock src/ $(SOURCE_FILES)
	cargo build --release

$(TARGET): target/release/$(TARGET)
	mv target/release/$@ .

.PHONY=clean
clean:
	rm -rf target/
	rm -f $(TARGET)

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
test-integration: target/release/$(TARGET)
	cargo test --test '*' --locked

.PHONY=test-unit
test-unit:
	cargo test --bins --locked

.PHONY=test
test: test-unit test-integration

.PHONY=precommit
precommit: lint test
