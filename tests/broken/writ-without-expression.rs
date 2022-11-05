use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{{ ` }}"]
struct Data {}

fn main() {
    print!("{}", Data {});
}
