ROOT_DIR := $(shell pwd)

rvemu: rvemu-wasm rvemu-cli

rvemu-wasm:
	wasm-pack build lib/rvemu-wasm --out-dir $(ROOT_DIR)/public/pkg --target web

rvemu-cli:
	cargo build --release --manifest-path lib/rvemu-cli/Cargo.toml

test: test-wasm test-isa

test-wasm:
	wasm-pack test lib/rvemu-wasm --headless --firefox

test-isa:
	cargo test

test-isa-debug:
	cargo test -- --nocapture

clean:
	cargo clean
