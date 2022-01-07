.PHONY: build test

build:
	cargo test --no-run --all-features 2>&1 | sed 's/ *--> sept\/\(.*\)/\1: error:/g'

test:
	RUST_BACKTRACE=1 RUST_LOG=debug cargo test --all-features -- --nocapture
