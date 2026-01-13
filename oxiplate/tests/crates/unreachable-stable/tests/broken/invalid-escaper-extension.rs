use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate = "template.htm"]
struct Data;

fn main() {
    Data.render().unwrap();
}
