use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "
{%- for value in &values -%}
    {{ value }}<br>
{%- endfor %}"]
struct Data {
    values: Vec<&'static str>,
}

#[test]
fn test_for() {
    let data = Data {
        values: vec!["foo", "bar", "baz"],
    };

    assert_eq!(format!("{}", data), "foo<br>bar<br>baz<br>");
}

#[derive(Oxiplate)]
#[oxiplate = "
{{- value }}!
{% for value in &values -%}
    {{ value }}
{% endfor -%}
{{ value }} again :D"]
struct ShadowVariable {
    values: Vec<&'static str>,
    value: &'static str,
}

#[test]
fn test_shadow_variable() {
    let data = ShadowVariable {
        values: vec!["foo", "bar", "baz"],
        value: "hello world",
    };

    assert_eq!(
        format!("{}", data),
        "hello world!
foo
bar
baz
hello world again :D"
    );
}
