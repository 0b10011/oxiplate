use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(html: "{{ var }}")]
struct Data {
    var: &'static str,
}

fn main() {
    print!("{}", Data { var: "foo" });
}
