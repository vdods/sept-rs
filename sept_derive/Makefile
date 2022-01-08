.PHONY: build test

build:
	cargo test --no-run --all-features 2>&1 | sed 's/ *--> sept_derive\/\(.*\)/\1: error:/g'
