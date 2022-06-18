use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "{{ ` }}"]
struct Data {}

fn main() {
    print!("{}", Data {});
}
