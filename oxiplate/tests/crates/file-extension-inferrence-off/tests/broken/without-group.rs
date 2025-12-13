use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate_inline("Message: {{ text: text}}")]
struct Data {
    value: &'static str,
}

fn main() {
    assert_eq!("Message: 1 < 3", Data { value: "1 < 3" }.render().unwrap());
}
