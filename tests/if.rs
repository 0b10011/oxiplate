use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxi_code = "{% if can_display %}Can {{ action }} :D{% else %}Can't {{ action }} :({% endif %}"]
struct Data {
    can_display: bool,
    action: &'static str,
}

#[test]
fn test_if() {
    let data = Data {
        can_display: true,
        action: "display",
    };

    assert_eq!(format!("{}", data), "Can display :D");
}

#[test]
fn test_else() {
    let data = Data {
        can_display: false,
        action: "display",
    };

    assert_eq!(format!("{}", data), "Can't display :(");
}
