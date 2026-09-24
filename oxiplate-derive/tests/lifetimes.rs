#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;

use oxiplate_derive::Oxiplate;

#[test]
fn one<'c>() {
    fn function<'a: 'a>(left: &'a str) -> String {
        format!("{left:?}")
    }

    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{{ function::<'c>(left) }}"#)]
    struct Data<'c> {
        left: &'c str,
    }

    assert_eq!(format!("{}", Data { left: "hello" }), r#""hello""#);
}

#[test]
fn one_trailing_comma<'c>() {
    fn function<'a: 'a>(left: &'a str) -> String {
        format!("{left:?}")
    }

    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{{ function::<'c,>(left) }}"#)]
    struct Data<'c> {
        left: &'c str,
    }

    assert_eq!(format!("{}", Data { left: "hello" }), r#""hello""#);
}

#[test]
fn two<'c, 'd>() {
    fn function<'a: 'a, 'b: 'b>(left: &'a str, right: &'b str) -> String {
        format!("{left:?} {right:?}")
    }

    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{{ function::<'c, 'd>(left, right) }}"#)]
    struct Data<'c, 'd> {
        left: &'c str,
        right: &'d str,
    }

    assert_eq!(
        format!(
            "{}",
            Data {
                left: "hello",
                right: "world",
            }
        ),
        r#""hello" "world""#
    );
}

#[test]
fn two_trailing_comma<'c, 'd>() {
    fn function<'a: 'a, 'b: 'b>(left: &'a str, right: &'b str) -> String {
        format!("{left:?} {right:?}")
    }

    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{{ function::<'c, 'd,>(left, right) }}"#)]
    struct Data<'c, 'd> {
        left: &'c str,
        right: &'d str,
    }

    assert_eq!(
        format!(
            "{}",
            Data {
                left: "hello",
                right: "world",
            }
        ),
        r#""hello" "world""#
    );
}
