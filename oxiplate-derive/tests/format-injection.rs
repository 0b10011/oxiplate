use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "Braces ({ and }) are formatting characters in Rust and must be escaped if used in formatting \
     strings. {}"
)]
struct Data {}

/// Ensure `{}` in a template doesn't break formatting.
#[test]
fn format_injection() {
    let data = Data {};

    assert_eq!(
        format!("{data}"),
        "Braces ({ and }) are formatting characters in Rust and must be escaped if used in \
         formatting strings. {}"
    );
}
