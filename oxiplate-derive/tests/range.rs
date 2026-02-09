#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{% if (..3).contains(&-1) %}3 contains -1{% endif %}
{% if (..b).contains(&-1) %}b contains -1{% endif %}
{% if (..3).contains(&3) %}3 contains 3{% endif %}
{% if (..b).contains(&3) %}b contains 3{% endif %}
{% if (..3).contains(&4) %}3 contains 4{% endif %}
{% if (..b).contains(&4) %}b contains 4{% endif %}
"
)]
struct RangeToExclusive {
    b: isize,
}

#[test]
fn range_to_exclusive() {
    assert_eq!(
        format!("{}", RangeToExclusive { b: 3 }),
        "
3 contains -1
b contains -1




"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{% if (..=3).contains(&-1) %}3 contains -1{% endif %}
{% if (..=b).contains(&-1) %}b contains -1{% endif %}
{% if (..=3).contains(&3) %}3 contains 3{% endif %}
{% if (..=b).contains(&3) %}b contains 3{% endif %}
{% if (..=3).contains(&4) %}3 contains 4{% endif %}
{% if (..=b).contains(&4) %}b contains 4{% endif %}
"
)]
struct RangeToInclusive {
    b: isize,
}

#[test]
fn range_to_inclusive() {
    assert_eq!(
        format!("{}", RangeToInclusive { b: 3 }),
        "
3 contains -1
b contains -1
3 contains 3
b contains 3


"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{% if (4..).contains(&3) %}4 contains 3{% endif %}
{% if (a..).contains(&3) %}a contains 3{% endif %}
{% if (4..).contains(&4) %}4 contains 4{% endif %}
{% if (a..).contains(&4) %}a contains 4{% endif %}
{% if (4..).contains(&127) %}4 contains 127{% endif %}
{% if (a..).contains(&127) %}a contains 127{% endif %}
"
)]
struct RangeFrom {
    a: i8,
}

#[test]
fn range_from() {
    assert_eq!(
        format!("{}", RangeFrom { a: 4 }),
        "


4 contains 4
a contains 4
4 contains 127
a contains 127
"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{% if (3..19).contains(&2) %}3 contains 2{% endif %}
{% if (a..b).contains(&2) %}ab contains 2{% endif %}
{% if (3..19).contains(&4) %}3 contains 3{% endif %}
{% if (a..b).contains(&4) %}ab contains 3{% endif %}
{% if (3..19).contains(&18) %}3 contains 18{% endif %}
{% if (a..b).contains(&18) %}ab contains 18{% endif %}
{% if (3..19).contains(&19) %}3 contains 19{% endif %}
{% if (a..b).contains(&19) %}ab contains 19{% endif %}
{% if (3..19).contains(&20) %}3 contains 20{% endif %}
{% if (a..b).contains(&20) %}ab contains 20{% endif %}
"
)]
struct RangeExclusive {
    a: i8,
    b: i8,
}

#[test]
fn range_exclusive() {
    assert_eq!(
        format!("{}", RangeExclusive { a: 3, b: 19 }),
        "


3 contains 3
ab contains 3
3 contains 18
ab contains 18




"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    "
{% if (3..=19).contains(&2) %}3 contains 2{% endif %}
{% if (a..=b).contains(&2) %}ab contains 2{% endif %}
{% if (3..=19).contains(&4) %}3 contains 3{% endif %}
{% if (a..=b).contains(&4) %}ab contains 3{% endif %}
{% if (3..=19).contains(&18) %}3 contains 18{% endif %}
{% if (a..=b).contains(&18) %}ab contains 18{% endif %}
{% if (3..=19).contains(&19) %}3 contains 19{% endif %}
{% if (a..=b).contains(&19) %}ab contains 19{% endif %}
{% if (3..=19).contains(&20) %}3 contains 20{% endif %}
{% if (a..=b).contains(&20) %}ab contains 20{% endif %}
"
)]
struct RangeInclusive {
    a: i8,
    b: i8,
}

#[test]
fn range_inclusive() {
    assert_eq!(
        format!("{}", RangeInclusive { a: 3, b: 19 }),
        "


3 contains 3
ab contains 3
3 contains 18
ab contains 18
3 contains 19
ab contains 19


"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"{-}
{{ a[..] _}}
{{ a[2..] _}}
{{ a[..2] _}}
{{ a[..=2] _}}
{{ a[2..4] _}}
{{ a[2..=4] -}}
"#
)]
struct RangeFull {
    a: &'static str,
}

#[test]
fn range_full() {
    assert_eq!(
        format!("{}", RangeFull { a: "abcde" }),
        "abcde cde ab abc cd cde"
    );
}
