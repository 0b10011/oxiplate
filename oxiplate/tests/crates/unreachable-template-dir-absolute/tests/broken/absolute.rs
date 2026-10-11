use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate = "external.html.oxip"]
struct Data;

fn main() {
    assert_eq!(Data.render().unwrap(), "Hello world!");
}
