use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r"{{ 'a' }}")]
struct A;

#[test]
fn a() {
    assert_eq!(format!("{}", A), "a");
}

#[derive(Oxiplate)]
#[oxiplate_inline(r"{{ '\'' }}")]
struct SingleQuote;

#[test]
fn single_quote() {
    assert_eq!(format!("{}", SingleQuote), "'");
}

#[derive(Oxiplate)]
#[oxiplate_inline(r"{{ '\\' }}")]
struct Slash;

#[test]
fn slash() {
    assert_eq!(format!("{}", Slash), r"\");
}
