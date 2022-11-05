use oxiplate::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "{{ foo bar }}"]
struct Data {
    foo: &'static str,
    bar: &'static str,
}

fn main() {
    print!("{}", Data { foo: "foo", bar: "bar" });
}
