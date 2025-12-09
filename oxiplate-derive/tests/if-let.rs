use oxiplate_derive::Oxiplate;

enum Name {
    Actual(String),
    Nickname { name: String },
    Missing,
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- if let Ok(name) = &name -%}
    {%- if let Some(cats_count) = cats_count -%}
        {%- if let Name::Actual(name) = name -%}
            Found {{ cats_count }} cats named {{ name }}!
        {%- elseif let Name::Nickname{name}=name -%}
            {# Extra whitespace intentionally skipped for coverage purposes -#}
            Found {{ cats_count }} cats nicknamed {{ name }}!
        {%- else -%}
            Found {{ cats_count }} cats!
        {%- endif -%}
    {%- else -%}
        {%- if let Name::Actual(missing_name) = &name -%}
            No cats named {{ missing_name }} found :(
        {%- elseif let Name::Nickname { name: missing_name } = &name -%}
            No cats nicknamed {{ missing_name }} found :(
        {%- else -%}
            No cats found :(
        {%- endif -%}
    {%- endif %}
{%- else -%}
    Name could not be fetched.
{%- endif -%}"
)]
struct Data {
    cats_count: Option<u8>,
    name: Result<Name, ()>,
}

#[test]
fn test_count() {
    let data = Data {
        cats_count: Some(5),
        name: Ok(Name::Missing),
    };

    assert_eq!(format!("{data}"), "Found 5 cats!");
}

#[test]
fn test_count_name() {
    let data = Data {
        cats_count: Some(5),
        name: Ok(Name::Actual(String::from("Sam"))),
    };

    assert_eq!(format!("{data}"), "Found 5 cats named Sam!");
}

#[test]
fn test_name() {
    let data = Data {
        cats_count: None,
        name: Ok(Name::Nickname {
            name: String::from("Sam"),
        }),
    };

    assert_eq!(format!("{data}"), "No cats nicknamed Sam found :(");
}

#[test]
fn test_none() {
    let data = Data {
        cats_count: None,
        name: Ok(Name::Missing),
    };

    assert_eq!(format!("{data}"), "No cats found :(");
}

struct Multiple {
    a: usize,
    b: char,
    c: &'static str,
    d: bool,
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- if let Multiple { a: 10,b:'b' , c: "19", d: false } = multiple -%}
    bad
{%- elseif let Multiple { a: 10,b:'b' , c: "19", d: true } = multiple -%}
    yes
{%- else -%}
    no
{%- endif -%}
"#
)]
struct MultipleWrapper {
    multiple: Multiple,
}

#[test]
fn test_multiple() {
    assert_eq!(
        "yes",
        format!(
            "{}",
            MultipleWrapper {
                multiple: Multiple {
                    a: 10,
                    b: 'b',
                    c: "19",
                    d: true
                }
            }
        )
    )
}
