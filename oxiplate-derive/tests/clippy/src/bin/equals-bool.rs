#![deny(clippy::bool_comparison)]

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline = "
{%- if foo == false -%}
    No foo
{%- endif %}"]
struct Data {
    foo: bool,
}

fn main() {
    let data = Data { foo: false };
    print!("{}", data);
}
