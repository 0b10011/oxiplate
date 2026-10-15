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

    pub fn respond_string(expression: String) -> impl Display {
        respond(expression)
    }
}

struct Message(String);

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ message | respond() }} {{ message | respond_string() }}"#)]
struct Data {
    message: Message,
}

fn main() {
    assert_eq!(
        format!(
            "{}",
            Data {
                message: Message("goodbye".to_string())
            }
        ),
        "did not understand: goodbye"
    );
}
