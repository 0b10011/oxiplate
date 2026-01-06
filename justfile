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
    cargo doc --locked --lib --no-deps

# Generate the book and open in your default browser.
[group("General Commands")]
book:
    mdbook serve --open

# Generate the book.
[group("General Commands")]
book-build:
    mdbook build

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

# Run tests without coverage.
[group("Test")]
test: (run-against-all "cargo test --locked") (run-against-libs "cargo test --locked --doc") expansion-tests

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
    (run-against-all "cargo llvm-cov --no-report --locked") \
    (run-against-libs "cargo llvm-cov --no-report --locked --doc")

[private]
expansion-tests:
    cargo test --locked --test expansion -- --ignored

[private]
clean-coverage:
    cargo llvm-cov clean --workspace

[private]
run-against-libs command:
    {{ command }} --package oxiplate-derive
    {{ command }} --workspace \
        --exclude oxiplate-derive \
        --exclude oxiplate-unreachable \
        --exclude oxiplate-derive-unreachable \
        --exclude oxiplate-test-fast-escape-type-priority \
        --exclude oxiplate-test-slow-escape-ints

[private]
run-against-all command: (run-against-libs command)
    {{ command }} --package oxiplate-test-fast-escape-type-priority
    {{ command }} --package oxiplate-test-slow-escape-ints
    {{ command }} --package oxiplate --test broken -- --ignored
    {{ command }} --package oxiplate-derive --test broken -- --ignored
    {{ command }} --package oxiplate-derive --test clippy -- --ignored
    {{ command }} --package oxiplate-unreachable
    {{ command }} --package oxiplate-derive-unreachable

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
    cargo install mdbook@0.4.52 mdbook-codename
