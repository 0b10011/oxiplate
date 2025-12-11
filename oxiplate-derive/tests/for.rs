use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- for a in &values -%}
    {%- for b in &values -%}
        {{ a ~ " - " ~ b }}<br>
    {%- endfor %}
{%- endfor %}"#
)]
struct Data {
    values: Vec<&'static str>,
}

#[test]
fn test_for() {
    let data = Data {
        values: vec!["foo", "bar"],
    };

    assert_eq!(
        format!("{data}"),
        "foo - foo<br>foo - bar<br>bar - foo<br>bar - bar<br>"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- for person in &people -%}
    {{ person.get_name() }}<br>
{%- endfor %}"
)]
struct Accounts {
    people: Vec<Person>,
}

struct Person {
    name: &'static str,
}
impl Person {
    pub fn get_name(&self) -> &'static str {
        self.name
    }
}

#[test]
fn test_method_calls() {
    let data = Accounts {
        people: vec![Person { name: "Zoe" }, Person { name: "Alice" }],
    };

    assert_eq!(format!("{data}"), "Zoe<br>Alice<br>");
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{{- value }}!
{% for value in &values -%}
    {{ value }}
{% endfor -%}
{{ value }} again :D"
)]
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
        format!("{data}"),
        "hello world!
foo
bar
baz
hello world again :D"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- for function in &functions -%}
    {{ function() }}
{% endfor %}"
)]
struct Functions {
    functions: Vec<fn() -> i32>,
}

#[test]
fn test_function_variables() {
    let data = Functions {
        functions: vec![|| 19, || 89],
    };

    assert_eq!(format!("{data}"), "19\n89\n");
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- for value in &values -%}
    {{ value }}<br>
{%- else -%}
    No values :(
{%- endfor %}"
)]
struct ForElse {
    values: Vec<&'static str>,
}

#[test]
fn test_for_else() {
    let data = ForElse { values: vec![] };

    assert_eq!(format!("{data}"), "No values :(");
}
