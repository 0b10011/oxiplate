use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "Braces ({ and }) are formatting characters in Rust and must be escaped. {}"]
struct Data {}

/// Ensure `{}` in a template doesn't break formatting.
#[test]
fn format_injection() {
    let template = Data {};

    assert_eq!(
        format!("{template}"),
        "Braces ({ and }) are formatting characters in Rust and must be escaped. {}"
    );
}
