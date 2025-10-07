#![doc(issue_tracker_base_url = "https://github.com/0b10011/oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod escapers;

pub use oxiplate_derive::Oxiplate;
pub use oxiplate_traits::{
    CowStr, CowStrWrapper, Escaper, FastCowStr, FastEscape, Render, ToCowStr, ToCowStrWrapper,
    UnescapedText, UnescapedTextWrapper,
};
