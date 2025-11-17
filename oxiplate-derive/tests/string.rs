use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r###"{{ ##"jane #"the deer"# doe"## }}"###)]
struct RawString {}

#[test]
fn raw_string() {
    let template = RawString {};

    assert_eq!(format!("{template}"), r###"jane #"the deer"# doe"###);
}
