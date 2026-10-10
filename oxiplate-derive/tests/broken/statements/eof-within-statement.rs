use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{% if true %}{# eof after this due to not being closed")]
struct Data {}

fn main() {
    print!("{}", Data {});
}
