use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline("{{ message }}")]
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
