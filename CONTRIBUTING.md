# Contributing to Oxiplate

Looking to use Oxiplate in your own project? Head on over to the [readme](./README.md) instead. Otherwise, read on!

## Initial setup

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Install command for code coverage with `cargo install cargo-llvm-cov`
3. (Optional.) Install `mdbook` and filename preprocessor with `cargo install mdbook mdbook-codename`
4. (Optional.) Install [`gnuplot`](http://www.gnuplot.info/) for benchmark charts via something like `apt install gnuplot`
5. [Fork this repository](https://docs.github.com/en/get-started/quickstart/fork-a-repo) and clone it to your machine
6. Run `cargo dev` in the root directory of the project to download [the dependencies](./Cargo.toml), run the tests, and watch for changes

## General development

`cargo dev` will watch the files for changes and run various lint tools and tests automatically while tracking coverage.
Test coverage reports can be found in `./target/llvm-cov/`.

`mdbook serve --open` will run a local webserver and open the book in your default browser.

## File organization

- [`/docs/`](./docs/) is the source for https://0b10011.io/oxiplate/
- [`/src/`](./src/) is where Oxiplate code lives; this is what processes `.oxip` templates and generates Rust code from them
- `/target/` will be created when you build the project for the first time; this is where the binaries and intermediate build files live
- [`/tests/`](./tests/) contains all of the tests to ensure Oxiplate continues to work as expected
- [`/tests/broken/`](./tests/broken/) contains tests specific to failures and the associated error messages
- [`/tests/expansion/`](./tests/expansion/) verifies macro expansion for all base tests.

## Testing

`cargo test` will run all but expansion tests. To include expansion tests, use `cargo test -- --ignored`. For more complicated failures, `cargo expand --test if` can be used to output the generated rust code for the `if` tests (replace `if` with the name of the test to expand).

There are three main categories of tests: features, failures, and expansions. Feature tests ensure the core features (e.g., if statements and whitespace control) work as expected. Failure tests use `trybuild` (via [`broken.rs`](./tests/broken.rs)) to ensure the error messages for broken builds are friendly to humans and actually help with debugging. And expansion tests use `cargo expand` (via [`expansion.rs`](./tests/expansion.rs)) to verify feature test expansion is happening as expected.
