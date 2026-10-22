use alloc::format;
use alloc::string::String;
use core::fmt::Debug;

use oxiplate_derive::Oxiplate;

fn format<T: Debug>(value: T) -> String {
    format!("{:?}", value)
}

#[test]
#[allow(unused_parens)]
fn string() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(
        r#"
{{ format(Into::<String>::into(value)) }}
{{ format(Into::<&str>::into(value)) }}
{{ format(Into::<&'a str>::into(value)) }}
{{ format(Into::<(String)>::into(value)) }}
{{ format(Into::<alloc::string::String>::into(value)) }}
{{ format(Into::<::alloc::string::String>::into(value)) }}
"#
    )]
    struct Data<'a> {
        value: &'a str,
    }

    assert_eq!(
        format!("{}", Data { value: "hello" }),
        r#"
"hello"
"hello"
"hello"
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

#[test]
fn inferred() {
    use alloc::vec::Vec;

    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{% for i in Vec::<_>::from([19, 89]) %}{{ i }}{% endfor %}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), "1989");
}

#[test]
fn qualified_path() {
    struct Foo;

    trait Bar {
        type Baz;
    }

    impl Bar for Foo {
        type Baz = u64;
    }

    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{{ Into::<<Foo as Bar>::Baz>::into(unsigned_16) + unsigned_64 }}"#)]
    struct Data {
        unsigned_16: u16,
        unsigned_64: u64,
    }

    assert_eq!(
        format!(
            "{}",
            Data {
                unsigned_16: 10_u16,
                unsigned_64: 9_u64
            }
        ),
        "19"
    );
}

#[test]
fn array() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{{ Into::<[u64; 2]>::into(values).iter().sum::<u64>() + unsigned_64 }}"#)]
    struct Data {
        values: (u64, u64),
        unsigned_64: u64,
    }

    assert_eq!(
        format!(
            "{}",
            Data {
                values: (80_u64, 9_u64),
                unsigned_64: 1900_u64
            }
        ),
        "1989"
    );
}

#[test]
fn tuple() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{{ Into::<(u64, u64)>::into(values).0 + Into::<(u64, u64)>::into(values).1 + unsigned_64 }}"#)]
    struct Data {
        values: [u64; 2],
        unsigned_64: u64,
    }

    assert_eq!(
        format!(
            "{}",
            Data {
                values: [80_u64, 9_u64],
                unsigned_64: 1900_u64
            }
        ),
        "1989"
    );
}
