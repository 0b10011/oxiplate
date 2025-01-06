#![deny(clippy::bool_comparison)]

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "
{%- if foo.is_empty() == false -%}
    No foo
{%- endif %}"]
struct Data {
    foo: &'static str,
}

fn main() {
    let data = Data { foo: "bar" };
    print!("{}", data);
}
