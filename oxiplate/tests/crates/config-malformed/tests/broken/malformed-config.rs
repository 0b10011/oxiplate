use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate_inline(html: "hello world")]
struct Data;

fn main() {
    assert_eq!(
        Data.render().unwrap(),
        "Hello world!"
    );
}
