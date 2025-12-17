[private]
default: (watch "dev")

# Watch files for changes and run the provided `just` command when there's a change. Typically used as `just watch dev`.
[group("General Commands")]
watch command:
    watchexec just {{ command }}

# Format code, run tests, generate coverage, and run clippy. Typically used via `just watch dev`.
[group("General Commands")]
dev: format coverage clippy expansion-tests doc

# Build documentation for libraries.
[group("General Commands")]
doc:
    cargo doc --lib --no-deps

# Generate the book and open in your default browser.
[group("General Commands")]
book:
    mdbook serve --open

# Format code.
[group("Lint")]
format:
    cargo fmt

# Run `cargo clippy` against all packages.
[group("Lint")]
clippy: (run-against-all "cargo clippy")

# Run tests without coverage.
[group("Test")]
test: (run-against-all "cargo test") expansion-tests

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

# Run tests with coverage without building a report. Typically used with `cargo llvm-cov report`.
[group("Test")]
coverage-no-report: clean-coverage (run-against-all "cargo llvm-cov --no-report")

[private]
expansion-tests:
    cargo test --test expansion -- --ignored

[private]
clean-coverage:
    cargo llvm-cov clean --workspace

[private]
run-against-all command:
    {{ command }} --package oxiplate-derive
    {{ command }} --package oxiplate-test-fast-escape-type-priority
    {{ command }} --package oxiplate-test-slow-escape-ints
    {{ command }} --workspace \
        --exclude oxiplate-derive \
        --exclude oxiplate-test-fast-escape-type-priority \
        --exclude oxiplate-test-slow-escape-ints

# Initial setup. Run once to install all necessary binaries. Run again to ensure they are all up-to-date.
[group("Setup"), group("General Commands")]
setup: setup-dev setup-expansion setup-coverage setup-book

# Initial setup for general development.
[group("Setup")]
setup-dev:
    cargo install just watchexec-cli

# Initial setup for expansion tests and debugging.
[group("Setup")]
setup-expansion:
    cargo install cargo-expand

# Initial setup for test coverage.
[group("Setup")]
setup-coverage:
    cargo install cargo-llvm-cov

# Initial setup for generating the book.
[group("Setup")]
setup-book:
    cargo install mdbook mdbook-codename
