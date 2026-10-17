use oxiplate::Oxiplate;

mod filters_for_oxiplate {
    use std::borrow::Cow;

    use oxiplate_traits::CowStr;

    pub fn respond<'a, E: CowStr<'a>, R: CowStr<'a>>(expression: E, response: R) -> Cow<'a, str> {
        let expression = expression.cow_str();
        let response = response.cow_str();

        match expression.as_ref() {
            "hello" => response,
            _ => format!("did not understand: {expression}").into(),
        }
    }
}

struct Foo;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ raw: >message | respond(>"world") }}"#)]
struct RespondString {
    message: Foo,
}

fn main() {
    assert_eq!(format!("{}", RespondString { message: Foo }), "world");
}
