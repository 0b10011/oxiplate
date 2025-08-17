use oxiplate::{Oxiplate, Render};

#[derive(Oxiplate)]
#[oxiplate = "extends-missing.html.oxip"]
struct Data;

fn main() {
    assert_eq!("", Data.render().unwrap());
}
