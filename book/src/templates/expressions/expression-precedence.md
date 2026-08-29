# Expression precedence

Expressions are computed in the following order:

1. All supported Rust expressions (fields, methods, index, operators, etc)
2. Concatenation (`expr ~ expr`)
3. Cow prefix (`>expr`)
4. Filters (`expr | filter`)

[Rust's precedence rules](https://doc.rust-lang.org/reference/expressions.html#expression-precedence) apply to all supported Rust expressions.
