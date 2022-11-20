use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = r#"{% if value == 5 %}bar{% endif %}"#]
struct Comparison {
    value: u8,
}

#[test]
fn test_5u8() {
    let data = Comparison { value: 5u8 };

    assert_eq!(format!("{}", data), "bar");
}

#[test]
fn test_5() {
    let data = Comparison { value: 5 };

    assert_eq!(format!("{}", data), "bar");
}

#[test]
fn test_4u8() {
    let data = Comparison { value: 4u8 };

    assert_eq!(format!("{}", data), "");
}

#[test]
fn test_4() {
    let data = Comparison { value: 4 };

    assert_eq!(format!("{}", data), "");
}
