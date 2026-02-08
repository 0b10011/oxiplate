#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "{-}
1 + 2 = {{ 1 + 2 }}
{{ max }} + {{ min }} = {{ max + min }}
{{ max }} - {{ min }} = {{ max - min }}
{{ max }} * {{ min }} = {{ max * min }}
{{ max }} / {{ min }} = {{ max / min }}
{{ max }} % {{ min }} = {{ max % min }}
{{ min }} + {{ min }} * {{ max }} = {{ min + min * max }}
{{ max }} + {{ max }} / {{ min }} = {{ max + max / min }}
{{ max }} - {{ min }} % {{ min }} = {{ max - min % min }}
{{ a }} - {{ b }} * {{ c }} = {{ a - b * c }}
{{ a }} / {{ b }} + {{ c }} = {{ a / b + c }}"
)]
struct Math {
    min: i16,
    max: i16,
    a: usize,
    b: usize,
    c: usize,
}

#[test]
fn test_math() {
    let data = Math {
        min: 19,
        max: 89,
        a: 16,
        b: 8,
        c: 2,
    };

    assert_eq!(
        format!("{data}"),
        "1 + 2 = 3
89 + 19 = 108
89 - 19 = 70
89 * 19 = 1691
89 / 19 = 4
89 % 19 = 13
19 + 19 * 89 = 1710
89 + 89 / 19 = 93
89 - 19 % 19 = 89
16 - 8 * 2 = 0
16 / 8 + 2 = 4"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "{-}
{{ max }} == {{ min }} = {{ max == min }}
{{ max }} != {{ min }} = {{ max != min }}
{{ max }} > {{ min }} = {{ max > min }}
{{ max }} < {{ min }} = {{ max < min }}
{{ max }} >= {{ min }} = {{ max >= min }}
{{ max }} <= {{ min }} = {{ max <= min }}"
)]
struct Comparisons {
    min: i16,
    max: i16,
}

#[test]
fn test_comparisons() {
    let data = Comparisons { min: 19, max: 89 };

    assert_eq!(
        format!("{data}"),
        "89 == 19 = false
89 != 19 = true
89 > 19 = true
89 < 19 = false
89 >= 19 = true
89 <= 19 = false"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "{-}
{{ yes }} || {{ yes }} = {{ yes || yes2 }}
{{ yes }} || {{ no }} = {{ yes || no }}
{{ no }} || {{ yes }} = {{ no || yes }}
{{ no }} || {{ no }} = {{ no || no2 }}
{{ yes }} && {{ yes }} = {{ yes && yes2 }}
{{ yes }} && {{ no }} = {{ yes && no }}
{{ no }} && {{ yes }} = {{ no && yes }}
{{ no }} && {{ no }} = {{ no && no2 }}
{{ yes }} || {{ no }} && {{ no }} = {{ yes || no && no2 }}
{{ no }} || {{ yes }} && {{ no }} = {{ no || yes && no2 }}
{{ no }} || {{ yes }} && {{ yes }} = {{ no || yes && yes2 }}"
)]
#[allow(clippy::struct_excessive_bools)]
struct OrAnd {
    yes: bool,
    yes2: bool,
    no: bool,
    no2: bool,
}

#[test]
fn test_or_and() {
    let data = OrAnd {
        yes: true,
        yes2: true,
        no: false,
        no2: false,
    };

    assert_eq!(
        format!("{data}"),
        "true || true = true
true || false = true
false || true = true
false || false = false
true && true = true
true && false = false
false && true = false
false && false = false
true || false && false = true
false || true && false = false
false || true && true = true"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{%- if a - b < c + b -%}
    {{ a - b }} < {{ c + b }}
{%- else -%}
    {{ a - b }} > {{ c + b }}
{%- endif -%}
"
)]
#[allow(clippy::struct_excessive_bools)]
struct OrderOfOperations {
    a: usize,
    b: usize,
    c: usize,
}

#[test]
fn test_order_of_operations() {
    let data = OrderOfOperations { a: 16, b: 8, c: 2 };

    assert_eq!(format!("{data}"), "8 < 10");
}
