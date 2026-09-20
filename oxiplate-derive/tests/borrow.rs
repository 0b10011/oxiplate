#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

struct User<'a> {
    name: &'a str,
}

#[derive(Oxiplate)]
#[oxiplate_inline("{{ user.name }}")]
struct Data<'a> {
    user: &'a User<'a>,
}

#[test]
fn field() {
    let name = "Liv";
    let user = User { name };
    let data = Data { user: &user };

    assert_eq!(format!("{data}"), "Liv");
}

#[test]
fn ref_and_deref() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(
        r#"
{%- if **&value == "a" -%}
    string slice
{%- elseif value == &"b" -%}
    borrowed string slice
{%- elseif &*value == &&*"c" -%}
    double borrowed and dereferenced string slice
{%- else -%}
    no match for "{{ value }}"
{%- endif -%}
"#
    )]
    struct Data {
        value: &'static &'static str,
    }

    assert_eq!(format!("{}", Data { value: &"a" }), "string slice");

    assert_eq!(format!("{}", Data { value: &"b" }), "borrowed string slice");

    assert_eq!(
        format!("{}", Data { value: &"c" }),
        "double borrowed and dereferenced string slice"
    );

    assert_eq!(format!("{}", Data { value: &"d" }), r#"no match for "d""#);
}

#[test]
fn test_ref_deref_assign() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(
        r#"
{%- let new_value = &**value -%}
{%- if new_value == "a" -%}
    A
{%- else -%}
    {{ new_value }}
{%- endif -%}
"#
    )]
    struct Data {
        value: &'static &'static str,
    }

    assert_eq!(format!("{}", Data { value: &"a" }), "A");

    assert_eq!(format!("{}", Data { value: &"b" }), "b");
}
