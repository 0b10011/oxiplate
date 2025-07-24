use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "extends.html.oxip"]
struct AbsoluteData {
    title: &'static str,
    message: &'static str,
}

#[test]
fn absolute() {
    let data = AbsoluteData {
        title: "Oxiplate Example",
        message: "Hello world!",
    };

    assert_eq!(
        format!("{data}"),
        "<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h1>Oxiplate Example</h1>\n  <p>Hello \
         world!</p>\n"
    );
}

#[test]
fn absolute_2() {
    let data = AbsoluteData {
        title: "Oxiplate Example #2",
        message: "Goodbye world!",
    };

    assert_eq!(
        format!("{data}"),
        "<!DOCTYPE html>\n<title>Oxiplate Example #2</title>\n<h1>Oxiplate Example #2</h1>\n  \
         <p>Goodbye world!</p>\n"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(html: r#"{% extends "extends-wrapper.html.oxip" %}
{% block content -%}
    <p>{{ message }}</p>
    {%- parent %}
{%- endblock %}
"#)]
struct Prefix {
    title: &'static str,
    message: &'static str,
}

#[test]
fn prefix() {
    let data = Prefix {
        title: "Prefixed block",
        message: "Hello world!",
    };

    assert_eq!(
        format!("{data}"),
        "<!DOCTYPE html>\n<title>Prefixed block</title>\n<p>Hello world!</p>test\n"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(html: r#"{% extends "extends-wrapper.html.oxip" %}
{% block content -%}
    <p>{{ message }}</p>
{%- endblock %}
"#)]
struct Replace {
    title: &'static str,
    message: &'static str,
}

#[test]
fn replace() {
    let data = Replace {
        title: "Replaced block",
        message: "Hello world!",
    };

    assert_eq!(
        format!("{data}"),
        "<!DOCTYPE html>\n<title>Replaced block</title>\n<p>Hello world!</p>\n"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(html: r#"{% extends "extends-wrapper.html.oxip" %}
{% block content -%}
    {% parent -%}
    <p>{{ message }}</p>
{%- endblock %}
"#)]
struct Suffix {
    title: &'static str,
    message: &'static str,
}

#[test]
fn suffix() {
    let data = Suffix {
        title: "Suffixed block",
        message: "Hello world!",
    };

    assert_eq!(
        format!("{data}"),
        "<!DOCTYPE html>\n<title>Suffixed block</title>\ntest<p>Hello world!</p>\n"
    );
}
