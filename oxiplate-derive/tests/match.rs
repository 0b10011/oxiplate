#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use core::fmt::Display;

use oxiplate_derive::Oxiplate;

enum Name {
    Actual(String),
    Nickname { name: String },
    Missing,
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- match (&name, cats_count) -%}
    {%- case ( Ok ( Name::Actual ( name ) ) , Some ( cats_count ) ) -%}
        {# Extra whitespace intentionally inserted for coverage purposes -#}
        Found {{ cats_count }} cats named {{ name }}!
    {%- case (Ok(Name::Actual(missing_name)), None) -%}
        No cats named {{ missing_name }} found :(
    {%- case (Ok(Name::Nickname { name }), Some(cats_count)) -%}
        {# Extra whitespace intentionally skipped for coverage purposes -#}
        Found {{ cats_count }} cats nicknamed {{ name }}!
    {%- case (Ok(Name::Nickname { name: missing_name }), None) -%}
        No cats nicknamed {{ missing_name }} found :(
    {%- case (Ok(Name::Missing), Some(cats_count)) -%}
        Found {{ cats_count }} cats!
    {%- case (Ok(Name::Missing), None) -%}
        No cats found :(
    {%- case (Err(_), _) -%}
        Name could not be fetched.
{%- endmatch -%}"
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

struct InnerA<T: Display> {
    value: T,
}
struct InnerB<T: Display>(T);

struct MiddleA<A: Display, B: Display> {
    a: InnerA<A>,
    b: InnerB<B>,
}

struct MiddleB<A: Display, B: Display>(InnerA<A>, InnerB<B>);

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- if let MiddleA { a: InnerA { value: 42 } , b: InnerB(b) } = a -%}
    {# Extra whitespace before comma intentional for coverage -#}
    a.b: {{ b }}
{%- elseif let MiddleB(InnerA { value: a } , InnerB(42.19)) = b -%}
    {# Extra whitespace before comma intentional for coverage -#}
    b.a: {{ a }}
{%- endif -%}
"#
)]
struct Outer {
    a: MiddleA<usize, f64>,
    b: MiddleB<usize, f64>,
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
    assert_eq!(format!("{}", outer!(42, 19.89, 89, 42.19)), "a.b: 19.89");
    assert_eq!(format!("{}", outer!(64, 19.89, 89, 42.19)), "b.a: 89");
    assert_eq!(format!("{}", outer!(64, 19.89, 89, 16.19)), "");
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- match value %}
    {%- case ..1 %}To 1
    {%- case ..=1 %}Through 1
    {%- case 2 %}2
    {%- case 3..4 %}3 to 4
    {%- case 3..=4 %}3 through 4
    {%- case 3.. %}3 and up
{%- endmatch -%}
"#
)]
struct RangeInteger {
    value: isize,
}

#[test]
fn range_integer() {
    assert_eq!("To 1", format!("{}", RangeInteger { value: 0 }));
    assert_eq!("Through 1", format!("{}", RangeInteger { value: 1 }));
    assert_eq!("2", format!("{}", RangeInteger { value: 2 }));
    assert_eq!("3 to 4", format!("{}", RangeInteger { value: 3 }));
    assert_eq!("3 through 4", format!("{}", RangeInteger { value: 4 }));
    assert_eq!("3 and up", format!("{}", RangeInteger { value: 5 }));
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- match value %}
    {%- case ..1. %}To 1
    {%- case ..=1. %}Through 1
    {%- case 2.0 %}2
    {%- case 3. ..4. %}3 to 4
    {%- case 3. ..=4. %}3 through 4
    {%- case 3. .. %}3 and up
    {%- case _ %}Something else
{%- endmatch -%}
"#
)]
struct RangeFloat {
    value: f64,
}

#[test]
fn range_float() {
    assert_eq!("To 1", format!("{}", RangeFloat { value: 0. }));
    assert_eq!("Through 1", format!("{}", RangeFloat { value: 1. }));
    assert_eq!("2", format!("{}", RangeFloat { value: 2. }));
    assert_eq!("Something else", format!("{}", RangeFloat { value: 2.19 }));
    assert_eq!("3 to 4", format!("{}", RangeFloat { value: 3. }));
    assert_eq!("3 through 4", format!("{}", RangeFloat { value: 4. }));
    assert_eq!("3 and up", format!("{}", RangeFloat { value: 5. }));
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- match value %}
    {%- case ..'b' %}To b
    {%- case ..='b' %}Through b
    {%- case 'c' %}c
    {%- case 'd'..'e' %}d to e
    {%- case 'd'..='e' %}d through e
    {%- case 'd'.. %}d and up
{%- endmatch -%}"#
)]
struct RangeChar {
    value: char,
}

#[test]
fn range_char() {
    assert_eq!("To b", format!("{}", RangeChar { value: 'a' }));
    assert_eq!("Through b", format!("{}", RangeChar { value: 'b' }));
    assert_eq!("c", format!("{}", RangeChar { value: 'c' }));
    assert_eq!("d to e", format!("{}", RangeChar { value: 'd' }));
    assert_eq!("d through e", format!("{}", RangeChar { value: 'e' }));
    assert_eq!("d and up", format!("{}", RangeChar { value: 'f' }));
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- match value %}
    {%- case 19 %}The best number
    {%- case 42 %}The answer
    {%- case 69 | 420 %}Internet number
    {%- case _ %}Boring number
{%- endmatch -%}
"#
)]
struct MultipleCases {
    value: usize,
}

#[test]
fn multiple_cases() {
    assert_eq!(
        "The best number",
        format!("{}", MultipleCases { value: 19 })
    );
    assert_eq!("The answer", format!("{}", MultipleCases { value: 42 }));
    assert_eq!(
        "Internet number",
        format!("{}", MultipleCases { value: 69 })
    );
    assert_eq!(
        "Internet number",
        format!("{}", MultipleCases { value: 420 })
    );
    assert_eq!("Boring number", format!("{}", MultipleCases { value: 794 }));
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{%- match value %}
    {%- case Some(number) if number % 2 == 0 %}Even
    {%- case Some(_) %}Odd
    {%- case None %}Missing
{%- endmatch -%}
"#
)]
struct Guard {
    value: Option<usize>,
}

#[test]
fn guard() {
    assert_eq!("Odd", format!("{}", Guard { value: Some(19) }));
    assert_eq!("Even", format!("{}", Guard { value: Some(42) }));
    assert_eq!("Missing", format!("{}", Guard { value: None }));
}
