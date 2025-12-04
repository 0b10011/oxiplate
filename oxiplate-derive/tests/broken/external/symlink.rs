use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "external-symlink.html.oxip"]
struct Data;

fn main() {
    print!("{}", Data);
}
