use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = r#"{
    "foo": "Hello {{ name }}!"
}"#]
struct Data<'a> {
    name: &'a str,
}

#[test]
fn variable() {
    let data = Data {
        name: r#"Fiona","bar":"Bobby Tables says 'hi'"#,
    };

    assert_eq!(
        format!("{data}"),
        r#"{
    "foo": "Hello Fiona\",\"bar\":\"Bobby Tables says 'hi'!"
}"#
    );
}
