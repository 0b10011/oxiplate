#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;

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
        {%- if let Name::Actual ( name ) = name -%}
            {# Extra whitespace intentionally inserted for coverage purposes -#}
            Found {{ cats_count }} cats named {{ name }}!
        {%- elseif let Name::Nickname{name}=name -%}
            {# Extra whitespace intentionally skipped for coverage purposes -#}
            Found {{ cats_count }} cats nicknamed {{ name }}!
        {%- else -%}
            Found {{ cats_count }} cats!
        {%- endif -%}
    {%- elseif let core::option::Option::None = cats_count -%}
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

struct InnerA {
    value: usize,
}
struct InnerB(usize);

struct MiddleA {
    a: InnerA,
    b: InnerB,
}

struct MiddleB(InnerA, InnerB);

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- if let MiddleA { a: InnerA { value: 42 } , b: InnerB(b) } = a -%}
    {# Extra whitespace before comma intentional for coverage -#}
    a.b: {{ b }}
{%- elseif let MiddleB(InnerA { value: a } , InnerB(42)) = b -%}
    {# Extra whitespace before comma intentional for coverage -#}
    b.a: {{ a }}
{%- endif -%}
"#
)]
struct Outer {
    a: MiddleA,
    b: MiddleB,
}

#[test]
fn nested() {
    macro_rules! a {
        ($a:literal, $b:literal) => {
            MiddleA {
                a: InnerA { value: $a },
                b: InnerB($b),
            }
        };
    }
    macro_rules! b {
        ($a:literal, $b:literal) => {
            MiddleB(InnerA { value: $a }, InnerB($b))
        };
    }
    macro_rules! outer {
        ($aa:literal, $ab:literal, $ba:literal, $bb:literal) => {
            Outer {
                a: a!($aa, $ab),
                b: b!($ba, $bb),
            }
        };
    }
    assert_eq!(format!("{}", outer!(42, 19, 89, 42)), "a.b: 19");
    assert_eq!(format!("{}", outer!(64, 19, 89, 42)), "b.a: 89");
    assert_eq!(format!("{}", outer!(64, 19, 89, 16)), "");
}
