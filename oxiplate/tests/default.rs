#![no_std]

use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline(html: r#"
<!DOCTYPE html>
<title>{{ title | default("Default title") }}</title>
{% if let Some(title) = title -%}
    <h1>{{ title }}</h1>
{% endif -%}
"#)]
struct Data {
    title: Option<&'static str>,
}

#[test]
fn some() {
    assert_eq!(
        r#"
<!DOCTYPE html>
<title>Hello world!</title>
<h1>Hello world!</h1>
"#,
        Data {
            title: Some("Hello world!"),
        }
        .render()
        .unwrap()
    )
}

#[test]
fn none() {
    assert_eq!(
        r#"
<!DOCTYPE html>
<title>Default title</title>
"#,
        Data { title: None }.render().unwrap()
    )
}
