ROOT_DIR := $(shell pwd)

rvemu: rvemu-wasm rvemu-cli

rvemu-wasm:
	wasm-pack build lib/rvemu-wasm --out-dir $(ROOT_DIR)/public/pkg --target web

rvemu-cli:
	cargo build --release --manifest-path lib/rvemu-cli/Cargo.toml

test: test-wasm

test-wasm:
	wasm-pack test lib/rvemu-wasm --headless --firefox

clean:
	cargo clean
