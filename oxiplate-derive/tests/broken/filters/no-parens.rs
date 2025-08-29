use oxiplate_derive::Oxiplate;

mod filters_for_oxiplate {
    use std::fmt::Display;

    pub fn extract_message(expression: super::Message) -> impl Display {
        expression.0
    }
}

struct Message(String);

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ message | extract_message }}"#)]
struct Data {
    message: Message,
}

fn main() {
    assert_eq!(format!("{}", Data { message: Message("goodbye".to_string()) }), "did not understand: goodbye");
}