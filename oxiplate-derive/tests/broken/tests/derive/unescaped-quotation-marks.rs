use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(html: "foo""bar")]
struct Data;

fn main() {}
