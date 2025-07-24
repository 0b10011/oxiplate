use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ name ~ " (" ~ company ~ ")" }}"#)]
struct User {
    name: &'static str,
    company: &'static str,
}

#[test]
fn variable() {
    let data = User {
        name: "Xavier",
        company: "XYZ Company",
    };

    assert_eq!(format!("{data}"), "Xavier (XYZ Company)");
}
