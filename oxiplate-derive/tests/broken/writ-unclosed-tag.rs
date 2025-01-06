use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{{ foo"]
struct Data {
    foo: &'static str,
}

fn main() {
    print!("{}", Data { foo: "foo" });
}
