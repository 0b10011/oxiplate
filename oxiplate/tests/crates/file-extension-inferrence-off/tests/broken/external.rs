use oxiplate::prelude::*;

#[derive(Oxiplate)]
#[oxiplate = "template.html.oxip"]
struct Data {
    text: &'static str,
}

fn main() {
    assert_eq!("Message: 1 < 3", Data { text: "1 < 3" }.render().unwrap());
}
