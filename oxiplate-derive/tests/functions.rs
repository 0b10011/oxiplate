#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[test]
fn self_function() {
    #[derive(Oxiplate)]
    #[oxiplate_inline("{{ Self::double(89) }} {{ self.double_value() }}")]
    struct Data {
        value: usize,
    }

    impl Data {
        fn double(value: usize) -> usize {
            value * 2
        }

        fn double_value(&self) -> usize {
            self.value * 2
        }
    }

    assert_eq!(format!("{}", Data { value: 19 }), "178 38");
}

#[test]
fn expression_function() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(
        r#"
{%- for i in 0..functions.len() -%}
    {{ functions[i](value) }}
{% endfor -%}
"#
    )]
    struct Data<'a> {
        functions: &'a [fn(usize) -> usize],
        value: usize,
    }

    assert_eq!(
        format!(
            "{}",
            Data {
                functions: &[|i| i - 1, |i| i, |i| i + 1, |i| i * 2],
                value: 19,
            }
        ),
        "18\n19\n20\n38\n"
    );
}

#[cfg(test)]
fn triple(value: usize) -> usize {
    value * 3
}

#[test]
fn crate_function() {
    #[derive(Oxiplate)]
    #[oxiplate_inline("{{ crate::triple(value) }}")]
    struct Data {
        value: usize,
    }

    assert_eq!(format!("{}", Data { value: 19 }), "57");
}

#[test]
fn global_path() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{{- ::core::borrow::Borrow::borrow(value) -}}"#)]
    struct Data {
        value: &'static str,
    }

    assert_eq!(
        format!(
            "{}",
            Data {
                value: "hello world"
            }
        ),
        "hello world"
    );
}
