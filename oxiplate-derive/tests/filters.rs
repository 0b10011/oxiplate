use oxiplate_derive::Oxiplate;

mod filters_for_oxiplate {
    use std::fmt::Display;

    pub fn respond(expression: impl Display, yell: bool) -> impl Display {
        let expression = expression.to_string();
        match expression.as_str() {
            "hello" => if yell { "WORLD" } else { "world" }.to_string(),
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

    pub fn trim(expression: impl Display) -> impl Display {
        expression.to_string().trim().to_owned()
    }

    pub fn replace(expression: impl Display, from: impl Display, to: impl Display) -> impl Display {
        expression
            .to_string()
            .replace(&from.to_string(), &to.to_string())
            .to_owned()
    }
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ message | respond(false) }} {{ message | respond(true) }}"#)]
struct Respond {
    message: &'static str,
}

#[test]
fn respond() {
    assert_eq!(format!("{}", Respond { message: "hello" }), "world WORLD");
    assert_eq!(
        format!("{}", Respond { message: "goodbye" }),
        "did not understand: goodbye did not understand: goodbye"
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

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ message | respond(false) | shorten(length) }}"#)]
struct Multiple {
    message: &'static str,
    length: usize,
}

#[test]
fn multiple() {
    assert_eq!(
        format!(
            "{}",
            Multiple {
                message: "hello",
                length: 6
            }
        ),
        "world"
    );
    assert_eq!(
        format!(
            "{}",
            Multiple {
                message: "hello",
                length: 5
            }
        ),
        "world"
    );
    assert_eq!(
        format!(
            "{}",
            Multiple {
                message: "hello",
                length: 4
            }
        ),
        "worl"
    );
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ value | trim }} {{ value | trim() }}"#)]
struct Trim {
    value: &'static str,
}

#[test]
fn trim() {
    assert_eq!("hi hi", format!("{}", Trim { value: " hi " }))
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ value | replace("ar", "oo") }}"#)]
struct Replace {
    value: &'static str,
}

#[test]
fn replace() {
    assert_eq!("boo boo", format!("{}", Replace { value: "bar bar" }))
}
