use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "this isn't a valid path"]
struct Data;

fn main() {
    assert_eq!(format!("{Data}"), "Hello world!");
}
