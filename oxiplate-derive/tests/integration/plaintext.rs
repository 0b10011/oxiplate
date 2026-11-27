use alloc::format;

use oxiplate_derive::Oxiplate;

#[test]
fn empty() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{% plaintext %}{% endplaintext %}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), "");
}

#[test]
fn text() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{% plaintext %}Hello world!{% endplaintext %}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), "Hello world!");
}

#[test]
fn leading_whitespace_remove() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{% plaintext -%}   Hello world!   {% endplaintext %}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), "Hello world!   ");
}

#[test]
fn leading_whitespace_collapse() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{% plaintext _%}   Hello world!   {% endplaintext %}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), " Hello world!   ");
}

#[test]
fn trailing_whitespace_remove() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{% plaintext %}   Hello world!   {%- endplaintext %}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), "   Hello world!");
}

#[test]
fn trailing_whitespace_collapse() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{% plaintext %}   Hello world!   {%_ endplaintext %}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), "   Hello world! ");
}

#[test]
fn writ() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{% plaintext %}Hello {{ "world" }}!{% endplaintext %}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), r#"Hello {{ "world" }}!"#);
}

#[test]
fn writ_broken() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{% plaintext %}{{ +_)(*&^%$#@!~"';',.><=-/?` }}{% endplaintext %}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), r#"{{ +_)(*&^%$#@!~"';',.><=-/?` }}"#);
}

#[test]
fn statement() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(
        r#"{% plaintext %}Hello {% if true %}world{% else %}friend{% endif %}!{% endplaintext %}"#
    )]
    struct Data;

    assert_eq!(
        format!("{Data}"),
        r#"Hello {% if true %}world{% else %}friend{% endif %}!"#
    );
}

#[test]
fn statement_broken() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{% plaintext %}{% if +_)(*&^%$#@!~"';',.><=-/?` %}{% endplaintext %}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), r#"{% if +_)(*&^%$#@!~"';',.><=-/?` %}"#);
}

#[test]
fn comment() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{% plaintext %}Hello {#- world #}!{% endplaintext %}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), r#"Hello {#- world #}!"#);
}

#[test]
fn comment_broken() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{% plaintext %}Hello {#- world!{% endplaintext %}"#)]
    struct Data;

    assert_eq!(format!("{Data}"), r#"Hello {#- world!"#);
}
