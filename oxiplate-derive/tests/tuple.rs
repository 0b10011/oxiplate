use oxiplate_derive::Oxiplate;

// #[derive(Oxiplate)]
// #[oxiplate_inline(
//     "
// {%- if let (a,) = (a,) -%}
//     {{ a }}
// {%- endif -%}
// "
// )]
// struct Single {
//     a: usize,
// }

#[test]
#[ignore = "Single tuple matching does not currently work properly."]
fn single() {
    // assert_eq!(format!("{}", Single { a: 9 }), "9");
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- if let (a, b) = (b, a) -%}
    {{ a }} + {{ b }} = {{ a + b -}}
{% endif -%}
"
)]
struct Double {
    a: usize,
    b: usize,
}

#[test]
fn double() {
    assert_eq!(format!("{}", Double { a: 10, b: 9 }), "9 + 10 = 19");
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- if let (a, b, c, d, e) = (e, d, c, b, a) -%}
    {{ a _}}
    {{ b _}}
    {{ c _}}
    {{ d _}}
    {{ e -}}
{% endif -%}
"
)]
struct Several {
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
}

#[test]
fn several() {
    assert_eq!(
        format!(
            "{}",
            Several {
                a: 5,
                b: 4,
                c: 3,
                d: 2,
                e: 1
            }
        ),
        "1 2 3 4 5"
    );
}
