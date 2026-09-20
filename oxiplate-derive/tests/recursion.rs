#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[test]
fn recursion() {
    #[derive(Oxiplate)]
    #[oxiplate_inline(r#"{{name}} ({% for child in children %}{{ child }}, {% endfor %})"#)]
    struct Person<'a> {
        name: &'a str,
        children: &'a [Person<'a>],
    }

    let jane = Person {
        name: "Jane",
        children: &[],
    };
    let john = Person {
        name: "John",
        children: &[jane],
    };
    assert_eq!(
        format!(
            "{}",
            Person {
                name: "Jen",
                children: &[john]
            }
        ),
        "Jen (John (Jane (), ), )"
    );
}
