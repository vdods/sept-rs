# The silly sed command is to make the error output of rustc parseable by Kate/KDevelop for easy error location clicking.
# See https://stackoverflow.com/questions/3618078/pipe-only-stderr-through-a-filter
# This setting of SHELL to /bin/bash is also necessary to get the pipe indirection to work.
SHELL = /bin/bash

.PHONY: build test

build:
	cargo test --no-run --all-features 2> >(sed 's/ *--> sept\/\(.*\)/\1: error:/g' >&2)

test:
	RUST_BACKTRACE=1 RUST_LOG=debug cargo test --all-features -- --nocapture
