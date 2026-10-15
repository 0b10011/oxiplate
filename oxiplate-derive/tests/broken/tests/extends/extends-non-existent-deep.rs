use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "extends-missing-deep.html.oxip"]
struct Data;

fn main() {
    assert_eq!("", format!("{}", Data));
}
