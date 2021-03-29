ROOT_DIR := $(shell pwd)

rvemu: rvemu-wasm rvemu-cli

rvemu-wasm:
	# Disable temporarily for this issue:
	# https://github.com/rustwasm/wasm-bindgen/issues/2508
	#rustup run nightly wasm-pack build lib/rvemu-wasm --out-dir $(ROOT_DIR)/public/pkg --target web --no-typescript

rvemu-cli:
	cargo build --release --manifest-path lib/rvemu-cli/Cargo.toml

test: test-wasm test-isa

test-wasm:
	# TODO: Fix wasm tests.
	#wasm-pack test lib/rvemu-wasm --headless --firefox

test-isa:
	RUST_BACKTRACE=1 cargo test -- --nocapture

clean:
	cargo clean
	rm -rf public/pkg
