use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "
{%- if do_this -%}
    This then {{ action }} :D
{%- else -%}
    Can't {{ action }} :(
{%- endif %}"]
struct Data {
    do_this: bool,
    action: &'static str,
}

#[test]
fn test_if() {
    let data = Data {
        do_this: true,
        action: "do something",
    };

    assert_eq!(format!("{}", data), "This then do something :D");
}

#[test]
fn test_else() {
    let data = Data {
        do_this: false,
        action: "do something",
    };

    assert_eq!(format!("{}", data), "Can't do something :(");
}
