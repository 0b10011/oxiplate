#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;

use oxiplate_derive::Oxiplate;

#[test]
fn lifetime<'b>() {
    fn function<'a: 'a>(data: &'a str) -> String {
        format!("{data:?}")
    }

    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{{ function::<'b>(message) }}"#)]
    struct Data<'b> {
        message: &'b str,
    }

    assert_eq!(
        format!(
            "{}",
            Data {
                message: "hello world"
            }
        ),
        r#""hello world""#
    );
}
