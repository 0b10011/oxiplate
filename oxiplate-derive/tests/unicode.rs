use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "unicode.html.oxip"]
struct Data {
    foo: &'static str,
}

#[test]
fn external_unicode() {
    let template = Data { foo: "bar" };

    assert_eq!(format!("{template}"), "bar❯\n");
}
