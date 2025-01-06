use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = r##"{% if ! %}"##]
struct NotAlone {}

fn main() {
    let data = NotAlone {};

    print!("{}", data);
}
