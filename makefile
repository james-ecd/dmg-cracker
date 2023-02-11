all: lint

lint:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings

.PHONY: lint