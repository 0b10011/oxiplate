use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate = "external.raw.oxip"]
struct Data {
    value: &'static str,
}

fn main() {
    assert_eq!("Hello World", Data { value: "Hello World" }.render().unwrap());
}
