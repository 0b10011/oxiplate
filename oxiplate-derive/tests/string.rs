use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r###"{{ ##"jane #"the deer"# doe"## }}"###)]
struct RawString {}

#[test]
fn raw_string() {
    let template = RawString {};

    assert_eq!(format!("{template}"), r###"jane #"the deer"# doe"###);
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ "" }}"#)]
struct EmptyString {}

#[test]
fn empty_string() {
    let template = EmptyString {};

    assert_eq!(format!("{template}"), "");
}
