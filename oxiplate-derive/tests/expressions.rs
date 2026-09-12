#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

mod filters_for_oxiplate {
    extern crate alloc;

    use alloc::string::ToString as _;
    use core::fmt::Display;

    pub fn respond(expression: impl Display, yell: bool) -> impl Display {
        let expression = expression.to_string();
        match expression.as_str() {
            "hello" => if yell { "WORLD" } else { "world" }.to_string(),
            _ => "did not understand: ".to_string() + &expression,
        }
    }
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ message | respond(*respond) ~ " " ~ (9 + 5 * 2) ~ " " ~ (message | respond(!respond)) }}"#)]
struct Respond {
    message: &'static str,
    respond: &'static bool,
}

#[test]
fn combination() {
    assert_eq!(
        format!(
            "{}",
            Respond {
                message: "hello",
                respond: &false
            }
        ),
        "world 19 WORLD"
    );
}
