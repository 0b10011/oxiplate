# Contributing to Oxiplate

Looking to use Oxiplate in your own project? Head on over to the [readme](./README.md) instead. Otherwise, read on!

## Initial setup

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. [Fork this repository](https://docs.github.com/en/get-started/quickstart/fork-a-repo) and clone it to your machine
3. Run `cargo test` in the root directory of the project to download [the dependencies](./Cargo.toml) and run [the tests](./tests/)

## File organization

- [`/docs/`](./docs/) is the source for https://0b10011.io/oxiplate/
- [`/src/`](./src/) is where Oxiplate code lives; this is what processes `.oxip` templates and generates Rust code from them
- `/target/` will be created when you build the project for the first time; this is where the binaries and intermediate build files live
- [`/tests/`](./tests/) contains all of the tests to ensure Oxiplate continues to work as expected
- [`/tests/broken/`](./tests/broken/) contains tests specific to failures and the associated error messages

## Testing

`cargo test` will run all tests. For more complicated failures, `cargo expand --test if` can be used to output the generated rust code for the `if` tests (replace `if` with the name of the test to expand).

There are two main categories of tests: features and failures. Feature tests ensure the core features (e.g., if statements and whitespace control) work as expected. Failure tests use `trybuild` (via [`broken.rs`](./broken.rs)) to ensure the error messages for broken builds are friendly to humans and actually help with debugging.
