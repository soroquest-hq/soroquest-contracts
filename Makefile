.PHONY: build test deploy-testnet fmt check

build:
	stellar contract build

test:
	cargo test --features testutils

fmt:
	cargo fmt --all

deploy-testnet:
	@echo "Run: stellar contract deploy --network testnet --wasm target/wasm32v1-none/release/soroquest_escrow.wasm"

check:
	cargo clippy --features testutils -- -D warnings
