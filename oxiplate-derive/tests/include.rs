#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% include "extends.html.oxip" %}"#)]
struct Include {
    title: &'static str,
    message: &'static str,
}

#[test]
fn include() {
    let data = Include {
        title: "Oxiplate Example",
        message: "Hello world!",
    };

    assert_eq!(
        format!("{}", data),
        "<!DOCTYPE html>\n<title>Oxiplate Example</title>\n<h1>Oxiplate Example</h1>\n  <p>Hello \
         world!</p>\n"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% include "include-deep.html.oxip" %}"#)]
struct IncludeDeep {
    title: &'static str,
    message: &'static str,
}

#[test]
fn include_deep() {
    let data = IncludeDeep {
        title: "Oxiplate Example",
        message: "Hello world!",
    };

    assert_eq!(
        format!("{}", data),
        "<h1>Oxiplate Example</h1>\n<p>foo</p>\n\n<p>Hello world!</p>\n"
    );
}
