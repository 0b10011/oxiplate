use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{% unreachable_escaper_group html %}")]
struct Data;

fn main() {
    print!("{}", Data);
}
