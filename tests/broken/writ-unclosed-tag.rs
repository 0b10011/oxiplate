use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate = "{{ foo"]
struct Data {
    foo: &'static str,
}

fn main() {
    print!("{}", Data { foo: "foo" });
}
