use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{{ ` }}"]
struct Data {}

fn main() {
    print!("{}", Data {});
}
