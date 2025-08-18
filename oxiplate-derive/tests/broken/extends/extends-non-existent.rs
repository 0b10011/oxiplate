use oxiplate_derive::{Oxiplate};

#[derive(Oxiplate)]
#[oxiplate = "extends-missing.html.oxip"]
struct Data;

fn main() {
    assert_eq!("", format!("{}", Data));
}
