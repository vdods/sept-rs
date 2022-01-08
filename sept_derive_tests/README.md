# sept_derive_tests

This is a bit silly, but the tests for `sept_derive` are in this separate crate because `cargo expand` is not working to show the macro-expanded output of the tests module.  And furthermore, the tests are declared within the library source code.

## How To Use

Install `cargo-expand`:

    cargo install cargo-expand

Run:

    cargo expand

and you should see the full macro-expanded source of this crate.
