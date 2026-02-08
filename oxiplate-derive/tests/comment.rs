#![no_std]

extern crate alloc;

use alloc::format;

use oxiplate_derive::Oxiplate;

#[derive(Oxiplate)]
#[oxiplate_inline("{# Hashes (#) are allowed anywhere in a comment #}")]
struct Hashes;

#[test]
fn hashes() {
    assert_eq!(format!("{}", Hashes), "");
}

#[derive(Oxiplate)]
#[oxiplate_inline("{# Other close tokens (`%}` and `}}`) should not affect parsing #}")]
struct TagEnds;

#[test]
fn tag_ends() {
    assert_eq!(format!("{}", TagEnds), "");
}
