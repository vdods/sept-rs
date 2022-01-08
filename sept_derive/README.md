# sept_derive

This crate was implemented using https://github.com/dtolnay/cargo-expand as a guide.

## Tests

The tests have been put in the crate `sept_derive_tests`, because developing proc macros for deriving traits requires use of the `cargo expand` tool to see the macro output for debugging purposes, but that tool doesn't work on the separate `tests` module that is in the common crate layout.
