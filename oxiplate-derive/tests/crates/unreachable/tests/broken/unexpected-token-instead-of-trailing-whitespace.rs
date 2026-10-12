//! Covers non-EOF invalid tokens
//! after trailing whitespace
//! following the end of a tag.
//! EOF encountered during the peek should be ignored,
//! but unexpected token errors should not be.

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline(r#"{{ "hello world" -}} ?"#)]
struct Data;

fn main() {
    print!("{Data}");
}
