use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- if !foo -%}
    Yay
{%- endif %}"
)]
struct Not {
    foo: bool,
}

#[test]
fn test_if() {
    let not = Not { foo: false };
    assert_eq!(format!("{not}"), "Yay");

    let not = Not { foo: true };
    assert_eq!(format!("{not}"), "");
}
