use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{{ 1 ;; 9 }}")]
struct Data;

fn main() {
    print!("{}", Data);
}
