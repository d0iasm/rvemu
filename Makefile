ROOT_DIR := $(shell pwd)

rvemu: rvemu-wasm rvemu-cli

rvemu-wasm:
	wasm-pack build lib/rvemu-wasm --out-dir $(ROOT_DIR)/public/pkg --target web --no-typescript

rvemu-cli:
	cargo build --release --manifest-path lib/rvemu-cli/Cargo.toml

test: test-wasm test-isa

test-wasm:
	# TODO: Fix wasm tests.
	#wasm-pack test lib/rvemu-wasm --headless --firefox

test-isa:
	cargo test -- --nocapture

clean:
	cargo clean
	rm -rf public/pkg
