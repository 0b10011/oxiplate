use alloc::format;

use oxiplate::prelude::*;

#[test]
fn basic_lower() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(
        r#"
{%- filter raw | lower -%}
    Hello World!
{%- endfilter -%}
"#
    )]
    struct Data;

    assert_eq!(format!("{Data}"), "hello world!");
}

#[test]
fn basic_lower_upper() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(
        r#"
{%- filter raw | >lower | upper -%}
    Hello World!
{%- endfilter -%}
"#
    )]
    struct Data;

    assert_eq!(format!("{Data}"), "HELLO WORLD!");
}

#[test]
fn writs_with_unsafe_chars() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(
        r#"
{%- filter html.text | >lower | upper -%}
    {{ message }}
{%- endfilter -%}
"#
    )]
    struct Data {
        message: &'static str,
    }

    assert_eq!(
        format!(
            "{}",
            Data {
                message: "<Hello> World!"
            }
        ),
        "&lt;HELLO> WORLD!"
    );
}
