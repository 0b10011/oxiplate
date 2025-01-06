use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{}"]
struct Data {}

/// Ensure `{}` in a template doesn't break formatting.
#[test]
fn format_injection() {
    let template = Data {};

    assert_eq!(format!("{template}"), "{}");
}
