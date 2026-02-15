#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use alloc::{format, vec};

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{{- value }}
{%- let value = 19 %}
{{ value }}
{%- let value = "89" %}
{{ value }}
"#
)]
struct Set {
    value: &'static str,
}

#[test]
fn set() {
    let data = Set {
        value: "Hello world!",
    };

    assert_eq!(format!("{data}"), "Hello world!\n19\n89\n");
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{{- value }}
{% if value == "Hello world!" -%}
    if
    {{_ value }}
    {%- let value = 19 %}
    {{_ value }}
{%- elseif value == "Goodbye world!" -%}
    elseif
    {{_ value }}
    {%- let value = 89 %}
    {{_ value }}
{%- else -%}
    else
    {{_ value }}
    {%- let value = 42 %}
    {{_ value }}
{%- endif %}
{{ value }}
"#
)]
struct ShadowIf {
    value: &'static str,
}

#[test]
fn shadow_if() {
    assert_eq!(
        format!(
            "{}",
            ShadowIf {
                value: "Hello world!"
            }
        ),
        "Hello world!\nif Hello world! 19\nHello world!\n"
    );
    assert_eq!(
        format!(
            "{}",
            ShadowIf {
                value: "Goodbye world!"
            }
        ),
        "Goodbye world!\nelseif Goodbye world! 89\nGoodbye world!\n"
    );
    assert_eq!(
        format!("{}", ShadowIf { value: "foobar" }),
        "foobar\nelse foobar 42\nfoobar\n"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{{- value }}
{% for number in &numbers %}
    {{- value }}
    {{_ number }}
    {%- let value = 19 %}
    {%- let number = 89 %}
    {{_ value }}
    {{_ number }}
{% endfor %}
{{- value }}
"#
)]
struct ShadowFor {
    value: &'static str,
    numbers: Vec<usize>,
}

#[test]
fn shadow_for() {
    assert_eq!(
        format!(
            "{}",
            ShadowFor {
                value: "Hello world!",
                numbers: vec![1, 2, 3]
            }
        ),
        "Hello world!\nHello world! 1 19 89\nHello world! 2 19 89\nHello world! 3 19 89\nHello \
         world!\n"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"
{{- value }}
{% match number %}
{% case 19 %}
    {{- value }}
    {%- let value = 19 %}
    {{_ value }}
{% case value %}
    {{- value }}
    {%- let value = "Goodbye world!" %}
    {{_ value }}
{% endmatch %}
{{- value }}
"#
)]
struct ShadowMatch {
    value: &'static str,
    number: usize,
}

#[test]
fn shadow_match() {
    assert_eq!(
        format!(
            "{}",
            ShadowMatch {
                value: "Hello world!",
                number: 19
            }
        ),
        "Hello world!\nHello world! 19\nHello world!\n"
    );
    assert_eq!(
        format!(
            "{}",
            ShadowMatch {
                value: "Hello world!",
                number: 89
            }
        ),
        "Hello world!\n89 Goodbye world!\nHello world!\n"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(
    r#"{% extends "let.html.oxip" %}
{% block content %}
    {{- value }}
    {%- let value = 69 %}
    {{_ value _}}
    |
    {%_ parent _%}
    |
    {{_ value }}
{%- endblock %}
{% block footer %}
    {{- value }}
    {%- let value = 420 %}
    {{_ value _}}
    |
    {%_ parent _%}
    |
    {{_ value }}
{%- endblock %}
"#
)]
struct Extends {
    value: &'static str,
}

#[test]
fn extends() {
    assert_eq!(
        format!("{}", Extends { value: "foo" }),
        r#"<!DOCTYPE html>
<header>foo</header>
<main>foo 69 | foo 19 | foo foo</main>
<footer>foo 420 | 42 89 | foo 42</footer>
"#
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% extends "let.html.oxip" %}"#)]
struct ExtendsDefault {
    value: &'static str,
}

#[test]
fn extends_default() {
    assert_eq!(
        format!("{}", ExtendsDefault { value: "foo" }),
        r#"<!DOCTYPE html>
<header>foo</header>
<main>foo 19 foo</main>
<footer>42 89 42</footer>
"#
    );
}

struct DestructureData(usize);

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% let DestructureData(value) = value %}{{ value }}"#)]
struct Destructure {
    value: DestructureData,
}

#[test]
fn destructure() {
    assert_eq!(
        format!(
            "{}",
            Destructure {
                value: DestructureData(19)
            }
        ),
        "19"
    );
}
