[private]
default: (watch "dev")

# Watch files for changes and run the provided `just` command when there's a change. Typically used as `just watch dev`.
[group("General Commands")]
watch command:
    watchexec just {{ command }}

# Format code, run tests, generate coverage, and run clippy. Typically used via `just watch dev`.
[group("General Commands")]
dev: format check coverage clippy expansion-tests doc

# Build documentation for libraries.
[group("General Commands")]
doc:
    cargo doc --locked --lib --no-deps

# Generate the book and open in your default browser.
[group("General Commands")]
book:
    mdbook serve --open

# Generate the book.
[group("General Commands")]
book-build:
    mdbook build

# Update expected output for broken and expansion tests to the current output.
[group("General Commands")]
update-test-output:
    # Update broken test results
    TRYBUILD=overwrite just run-against-broken "cargo test --locked"

    # Update config test output
    [ ! -f ./oxiplate-derive/tests/config/crates/*/tests/broken/*.stderr ] || cp ./oxiplate-derive/tests/config/crates/*/tests/broken/*.stderr ./oxiplate-derive/tests/config/expected/

    # Update expansion test results
    cargo test --locked --test expansion --features better-errors --no-fail-fast -- --ignored || true
    [ ! -f ./oxiplate/tests/expansion/actual/*.rs ] || mv ./oxiplate/tests/expansion/actual/*.rs ./oxiplate/tests/expansion/expected/
    [ ! -f ./oxiplate-derive/tests/expansion/actual/*.rs ] || mv ./oxiplate-derive/tests/expansion/actual/*.rs ./oxiplate/tests/expansion/expected/

    echo "Test output updated successfully!"

# Build (or rebuild) test crates for config via `oxiplate.toml`
[group("General Commands")]
build-config-test-crates:
    cargo run --bin oxiplate-derive-test-config
    cargo update --workspace

# Run book tests.
book-tests:
    cargo build --package oxiplate --target-dir target/book/
    RUSTUP_TOOLCHAIN="nightly-2026-02-05" CARGO_MANIFEST_DIR=`pwd`/book-lib mdbook test --library-path target/book/debug/deps

# Format code.
[group("Lint")]
format: && (format-broken "rustfmt")
    cargo fmt

# Check if code is formatted properly. Use `just format` to format automatically.
[group("Lint")]
format-check: && (format-broken "rustfmt --check")
    cargo fmt --check

# Format Rust files in each `/broken/` directory
[private]
[group("Lint")]
format-broken command:
    find -type d -name broken -print0 | xargs -0 -I{} find '{}' -name '*.rs' -print0 | xargs -0 -n 999 {{ command }}

# Run `cargo clippy` against all packages.
[group("Lint")]
clippy:
    cargo clippy --locked --workspace

# Run check against all packages.
[group("Test")]
check: (run-against-stable "cargo check --locked" "") (run-against-unstable "cargo check --locked" "")

# Run check against all packages, denying warnings.
[group("Test")]
check-strict: (run-against-stable "RUSTFLAGS='-D warnings' cargo check --locked" "") (run-against-unstable "RUSTFLAGS='-D warnings' cargo check --locked" "")

# Run tests without coverage.
[group("Test")]
test: (run-against-all "cargo test --locked") (run-against-libs "cargo test --locked --doc") book-tests expansion-tests
    cargo test --package oxiplate-derive --test clippy -- --ignored

# Run stable tests against specified toolchain (target triples not supported).
[group("Test")]
[arg("toolchain", pattern='(stable|beta|nightly|\d+\.\d+(\.\d+)?(-beta(\.\d+)?)?)(-\d{4}-\d{2}-\d{2})?')]
test-toolchain toolchain: && (run-against-stable f"cargo +{{ toolchain }} test --locked")
    @echo "Running tests against {{ toolchain }} toolchain..."

# Run stable tests against `rust-version` listed in `/Cargo.toml`.
[group("Test")]
test-msrv: (test-toolchain `just get-msrv`)

[private]
get-msrv:
    @cargo metadata --no-deps --format-version 1 \
    | jq --join-output '.packages[] | select(.name == "oxiplate") | .rust_version'

# Build HTML and LCOV reports from running tests with coverage.
[group("Test")]
coverage: coverage-lcov coverage-html

# Build LCOV report from running tests with coverage.
[group("Test")]
coverage-lcov: coverage-no-report
    cargo llvm-cov report --lcov --output-path lcov.info

# Build HTML report from running tests with coverage.
[group("Test")]
coverage-html: coverage-no-report
    cargo llvm-cov report --html

# Build LCOV report for each package from running tests with coverage.
[group("Test")]
coverage-lcov-packages: coverage-no-report \
    (coverage-lcov-package "oxiplate" "oxiplate-derive|oxiplate-traits") \
    (coverage-lcov-package "oxiplate-derive" "oxiplate|oxiplate-traits") \
    (coverage-lcov-package "oxiplate-traits" "oxiplate|oxiplate-derive")

[private]
[group("Test")]
coverage-lcov-package package other-packages: coverage-no-report
    cargo llvm-cov report --ignore-filename-regex "^$PWD/({{ other-packages }})/" --lcov --output-path {{ package }}.lcov

# Run tests with coverage without building a report. Typically used with `cargo llvm-cov report`.
[group("Test")]
coverage-no-report: clean-coverage \
    (run-against-all "cargo llvm-cov --no-report --locked --no-rustc-wrapper") \
    (run-against-libs "cargo llvm-cov --no-report --locked --no-rustc-wrapper --doc")
    cargo test --package oxiplate-derive --test clippy -- --ignored

[private]
expansion-tests:
    cargo test --locked --test expansion --features better-errors -- --ignored

[private]
clean-coverage:
    cargo llvm-cov clean --workspace

[private]
run-against-libs command test-arguments="":
    {{ command }} --package oxiplate-derive -- {{ test-arguments }}
    {{ command }} --workspace \
        --exclude oxiplate-derive \
        --exclude oxiplate-derive-test-unreachable \
        --exclude oxiplate-derive-test-unreachable-stable \
        --exclude oxiplate-test-fast-escape-type-priority \
        --exclude oxiplate-test-file-extension-inferrence-off \
        --exclude oxiplate-test-slow-escape-ints \
        --exclude oxiplate-test-unreachable \
        --exclude oxiplate-test-unreachable-stable -- {{ test-arguments }}

[private]
run-against-all command test-arguments="": (run-against-stable command test-arguments) (run-against-unstable command test-arguments) check-strict (run-against-broken command)

# Tests that can be run against the MSRV in `/Cargo.toml`
# and the nightly specified in `/rust-toolchain.toml`.
[private]
run-against-stable command test-arguments="": (run-against-libs command test-arguments)
    {{ command }} --package oxiplate-test-fast-escape-type-priority -- {{ test-arguments }}
    {{ command }} --package oxiplate-test-slow-escape-ints -- {{ test-arguments }}

# Tests requiring unstable features that cannot be run against the MSRV.
[private]
run-against-unstable command test-arguments="":
    {{ command }} --package oxiplate-test-unreachable -- {{ test-arguments }}
    {{ command }} --package oxiplate-test-unreachable-stable -- {{ test-arguments }}
    {{ command }} --package oxiplate-test-file-extension-inferrence-off -- {{ test-arguments }}
    {{ command }} --package oxiplate-derive-test-unreachable -- {{ test-arguments }}
    {{ command }} --package oxiplate-derive-test-unreachable-stable -- {{ test-arguments }}

# Tests for broken compilations.
[private]
run-against-broken command: (run-against-libs f"{{ command }} --test broken --features external-template-spans" "--ignored") (run-against-unstable f"{{ command }} --test broken" "--ignored")

# Initial setup. Run once to install all necessary binaries. Run again to ensure they are all up-to-date.
[group("Setup"), group("General Commands")]
setup: setup-dev setup-test setup-expansion setup-coverage setup-book

# Initial setup for general development.
[group("Setup")]
setup-dev:
    cargo install just watchexec-cli

# Initial setup for testing
[group("Setup")]
setup-test: build-config-test-crates

# Initial setup for expansion tests and debugging.
[group("Setup")]
setup-expansion:
    cargo install cargo-expand

# Initial setup for test coverage.
[group("Setup")]
setup-coverage: setup-test
    cargo install cargo-llvm-cov

# Initial setup for generating the book.
[group("Setup")]
setup-book:
    cargo install mdbook@0.4.52 mdbook-codename
