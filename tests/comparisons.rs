use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = r#"{% if value == "foo" %}bar{% endif %}"#]
struct Comparison {
    value: &'static str,
}

#[test]
fn test_equals_string() {
    let data = Comparison { value: "foo" };

    assert_eq!(format!("{}", data), "bar");
}

#[test]
fn test_does_not_equal_string() {
    let data = Comparison { value: "baz" };

    assert_eq!(format!("{}", data), "");
}
