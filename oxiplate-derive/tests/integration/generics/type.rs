use alloc::format;
use alloc::string::String;
use core::fmt::Debug;

use oxiplate_derive::Oxiplate;

fn format<T: Debug>(value: T) -> String {
    format!("{:?}", value)
}

#[test]
fn string() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(
        r#"
{{ format(Into::<String>::into(value)) }}
{{ format(Into::<alloc::string::String>::into(value)) }}
{{ format(Into::<::alloc::string::String>::into(value)) }}
"#
    )]
    struct Data {
        value: &'static str,
    }

    assert_eq!(
        format!("{}", Data { value: "hello" }),
        r#"
"hello"
"hello"
"hello"
"#
    );
}

#[test]
fn cow() {
    use alloc::borrow::Cow;

    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{{ format(Into::<Cow<str>>::into(value)) }}"#)]
    struct Data {
        value: &'static str,
    }

    assert_eq!(format!("{}", Data { value: "hello" }), r#""hello""#);
}

#[test]
fn integer() {
    use alloc::string::ToString;

    #[derive(Oxiplate)]
    #[oxiplate_inline(
        r#"
{{ Into::<u64>::into(unsigned_32) + unsigned_64 }}
{{ unsigned_32.to_string().parse::<u64>().unwrap() + unsigned_64 }}
"#
    )]
    struct Data {
        unsigned_32: u32,
        unsigned_64: u64,
    }

    assert_eq!(
        format!(
            "{}",
            Data {
                unsigned_32: 10_u32,
                unsigned_64: 9_u64
            }
        ),
        "
19
19
"
    );
}
