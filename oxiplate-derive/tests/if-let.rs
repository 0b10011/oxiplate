use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- if let Some(count) = cats_count -%}
    {%- if let Some(name) = name -%}
        Found {{ count }} cats named {{ name }}!
    {%- else -%}
        Found {{ count }} cats!
    {%- endif -%}
{%- else -%}
    {%- if let Some(missing_name) = name -%}
        No cats named {{ missing_name }} found :(
    {%- else -%}
        No cats found :(
    {%- endif -%}
{%- endif %}"
)]
struct Data {
    cats_count: Option<u8>,
    name: Option<String>,
}

#[test]
fn test_count() {
    let data = Data {
        cats_count: Some(5),
        name: None,
    };

    assert_eq!(format!("{data}"), "Found 5 cats!");
}

#[test]
fn test_count_name() {
    let data = Data {
        cats_count: Some(5),
        name: Some(String::from("Sam")),
    };

    assert_eq!(format!("{data}"), "Found 5 cats named Sam!");
}

#[test]
fn test_name() {
    let data = Data {
        cats_count: None,
        name: Some(String::from("Sam")),
    };

    assert_eq!(format!("{data}"), "No cats named Sam found :(");
}

#[test]
fn test_none() {
    let data = Data {
        cats_count: None,
        name: None,
    };

    assert_eq!(format!("{data}"), "No cats found :(");
}
