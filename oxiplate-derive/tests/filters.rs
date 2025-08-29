use oxiplate_derive::Oxiplate;

mod filters_for_oxiplate {
    use std::fmt::Display;

    pub fn respond(expression: impl Display) -> impl Display {
        let expression = expression.to_string();
        match expression.as_str() {
            "hello" => "world".to_string(),
            _ => "did not understand: ".to_string() + &expression,
        }
    }

    pub fn shorten(expression: impl Display, max_length: usize) -> impl Display {
        let expression = expression.to_string();
        if expression.len() <= max_length {
            expression
        } else {
            expression[0..=max_length - 1].to_string()
        }
    }

    pub fn pad(expression: usize, max_length: usize) -> impl Display {
        format!("{:width$}", expression, width = max_length)
    }
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ message | respond() }}"#)]
struct Respond {
    message: &'static str,
}

#[test]
fn respond() {
    assert_eq!(format!("{}", Respond { message: "hello" }), "world");
    assert_eq!(
        format!("{}", Respond { message: "goodbye" }),
        "did not understand: goodbye"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ message | shorten(max_length) }}"#)]
struct Shorten {
    message: &'static str,
    max_length: usize,
}

#[test]
fn shorten() {
    assert_eq!(
        format!(
            "{}",
            Shorten {
                message: "hello",
                max_length: 2
            }
        ),
        "he"
    );
    assert_eq!(
        format!(
            "{}",
            Shorten {
                message: "goodbye",
                max_length: 3
            }
        ),
        "goo"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ number | pad(length) }}"#)]
struct Pad {
    number: usize,
    length: usize,
}

#[test]
fn pad() {
    assert_eq!(
        format!(
            "{}",
            Pad {
                number: 19,
                length: 2
            }
        ),
        "19"
    );
    assert_eq!(
        format!(
            "{}",
            Pad {
                number: 19,
                length: 3
            }
        ),
        " 19"
    );
}
