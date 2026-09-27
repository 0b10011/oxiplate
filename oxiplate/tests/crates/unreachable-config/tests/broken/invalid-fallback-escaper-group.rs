use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline("{{ text: message }}")]
struct Data {
    message: &'static str,
}

fn main() {
    Data {
        message: "hello world!",
    }
    .render()
    .unwrap();
}
