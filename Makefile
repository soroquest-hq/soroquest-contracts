.PHONY: all build build-wasm test fmt clippy clean verify deploy-testnet help

all: test build-wasm

build:
	cargo build

build-wasm:
	cargo build --target wasm32v1-none --release --locked

test:
	cargo test --locked

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets -- -D warnings

verify: test clippy fmt-check
	@echo "All contract verification checks passed."

clean:
	cargo clean

deploy-testnet:
	./scripts/deploy_testnet.sh

help:
	@echo "SoroQuest Contract Development Tasks:"
	@echo "  make build-wasm       - Build release WASM for target wasm32v1-none"
	@echo "  make test             - Execute all unittests and invariant suites"
	@echo "  make verify           - Run tests, clippy, and fmt checks"
	@echo "  make deploy-testnet   - Deploy optimized WASM to Stellar testnet"
