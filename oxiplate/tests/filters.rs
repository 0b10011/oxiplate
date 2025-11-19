use oxiplate_derive::Oxiplate;

mod filters_for_oxiplate {
    use std::borrow::Cow;

    use oxiplate::CowStr;

    pub fn respond<'a, E: CowStr<'a>, R: CowStr<'a>>(expression: E, response: R) -> Cow<'a, str> {
        let expression = expression.cow_str();
        let response = response.cow_str();

        match expression.as_ref() {
            "hello" => response,
            _ => format!("did not understand: {expression}").into(),
        }
    }

    pub fn shorten<'a, E: CowStr<'a>>(expression: E, max_length: usize) -> Cow<'a, str> {
        let expression = expression.cow_str();

        if expression.len() <= max_length {
            expression
        } else {
            match expression {
                Cow::Borrowed(expression) => Cow::Borrowed(&expression[0..=max_length - 1]),
                Cow::Owned(expression) => Cow::Owned(expression[0..=max_length - 1].to_owned()),
            }
        }
    }

    pub fn pad(expression: usize, max_length: usize) -> Cow<'static, str> {
        format!("{:width$}", expression, width = max_length).into()
    }
}

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ raw: >message | >respond(>"world") }}"#)]
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
#[oxiplate_inline(r#"{{ raw: >message | shorten(max_length) }}"#)]
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
#[oxiplate_inline(r#"{{ raw: number | pad(length) }}"#)]
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
#[oxiplate_inline(r#"{{ raw: >message | >respond(>"world") | shorten(length) }}"#)]
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
