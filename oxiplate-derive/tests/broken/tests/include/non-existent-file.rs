use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{% include "this-file-does-not-exist.oxip" %}"#)]
struct Data;

fn main() {
    print!("{}", Data);
}
