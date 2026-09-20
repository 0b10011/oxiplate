#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[test]
fn test_assignment() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{%- let array = [ "a" ;  3 ] -%}{{ array.join("") }}"#)]
    struct Array;

    assert_eq!(format!("{}", Array), "aaa");
}

#[test]
fn test_array_of_string_slices_in_writ() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"[{{ ["a", "b", "c"].join("") }}]"#)]
    struct Array;

    assert_eq!(format!("{}", Array), "[abc]");
}

#[test]
fn test_array_of_numbers_in_for_loop() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(
        r#"[
            {%- for elem in [1, 2, 3] -%}
                {{ elem }}
            {%- endfor -%}
        ]"#
    )]
    struct Array;

    assert_eq!(format!("{}", Array), "[123]");
}

#[test]
fn test_array_of_expressions() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"[{{ ["a", "b", &"c".to_uppercase()].join("") }}]"#)]
    struct Array;

    assert_eq!(format!("{}", Array), "[abC]");
}

#[test]
fn test_repeat() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"[{{ ["a"; 3].join("") }}]"#)]
    struct Array;

    assert_eq!(format!("{}", Array), "[aaa]");
}

#[test]
fn test_repeat_empty() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"[{{ ["a"; 0].join("") }}]"#)]
    struct Array;

    assert_eq!(format!("{}", Array), "[]");
}
