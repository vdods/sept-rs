# The silly sed command is to make the error output of rustc parseable by Kate/KDevelop for easy error location clicking.
# See https://stackoverflow.com/questions/3618078/pipe-only-stderr-through-a-filter
# This setting of SHELL to /bin/bash is also necessary to get the pipe indirection to work.
SHELL = /bin/bash

.PHONY: sept sept_test sept-cat

sept:
	cargo test --package sept --no-run --all-features 2> >(sed 's/ *--> \(.*\)/\1: error:/g' >&2)

sept_test:
	RUST_BACKTRACE=1 RUST_LOG=debug cargo test --package sept --all-features -- --nocapture 2> >(sed 's/ *--> \(.*\)/\1: error:/g' >&2)

sept-cat:
	cargo build --package sept-cat --all-features 2> >(sed 's/ *--> \(.*\)/\1: error:/g' >&2)
