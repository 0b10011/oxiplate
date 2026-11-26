use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate = "json-with-different-extension.tmpl"]
struct Json {
    name: &'static str,
}

#[test]
fn json() {
    assert_eq!(
        Json {
            name: r#"Jane "JSON" Sonder"#
        }
        .render()
        .unwrap(),
        r#"{
    "foo": "hello Jane \"JSON\" Sonder",
    "bar": "goodbye Jane \"JSON\" Sonder"
}
"#
    );
}
