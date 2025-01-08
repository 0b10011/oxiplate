use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "\
    hello \
    world"]
struct Data {}

#[test]
fn external_unicode() {
    let template = Data {};

    assert_eq!(format!("{template}"), "hello world");
}
