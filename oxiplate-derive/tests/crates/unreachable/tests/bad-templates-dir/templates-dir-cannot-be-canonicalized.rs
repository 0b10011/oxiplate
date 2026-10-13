use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "page.oxip"]
struct Data;

fn main() {
    print!("{Data}");
}
