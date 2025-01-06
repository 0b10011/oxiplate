use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = r##"{% if "foo" == 3 %}"##]
struct Data {}

fn main() {
    let data = Data {};

    print!("{}", data);
}
