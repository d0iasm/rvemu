ROOT_DIR := $(shell pwd)

rvemu: rvemu-wasm rvemu-cli

rvemu-wasm:
	rustup run nightly wasm-pack build lib/rvemu-wasm --out-dir $(ROOT_DIR)/public/pkg --target web --no-typescript

rvemu-cli:
	cargo build --release --manifest-path lib/rvemu-cli/Cargo.toml

test:
	RUST_BACKTRACE=1 cargo test -- --nocapture --test-threads=1

clean:
	cargo clean
	rm -rf public/pkg
