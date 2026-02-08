#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- if !ref.is_empty() -%}
    Referee: {{ ref }}
{%- else -%}
    {{ else }}
{%- endif %}"
)]
struct Data {
    r#ref: &'static str,
    r#else: &'static str,
}

#[test]
fn test_if() {
    let data = Data {
        r#ref: "Jax",
        r#else: "No referee available",
    };

    assert_eq!(format!("{data}"), "Referee: Jax");
}

#[test]
fn test_else() {
    let data = Data {
        r#ref: "",
        r#else: "No referee available",
    };

    assert_eq!(format!("{data}"), "No referee available");
}

#[test]
fn syn_tokens() {}
