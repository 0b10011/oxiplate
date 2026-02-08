#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

enum Type {
    Text(&'static str),
    Numbers(u8, u8),
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r"
{%- if let Type::Text(text) = ty -%}
{{ text }}
{%- elseif let Type::Numbers(left, right) = ty -%}
{{ left }} + {{ right }} = {{ left + right }}
{%- endif -%}
"
)]
struct Data {
    ty: Type,
}

#[test]
fn text() {
    let data = Data {
        ty: Type::Text("foo"),
    };

    assert_eq!(format!("{data}"), "foo");
}

#[test]
fn numbers() {
    let data = Data {
        ty: Type::Numbers(10, 9),
    };

    assert_eq!(format!("{data}"), "10 + 9 = 19");
}
