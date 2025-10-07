#![doc(issue_tracker_base_url = "https://github.com/0b10011/oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod cow_str;
mod escaper;
mod render;
mod unescaped_text;

pub use cow_str::{CowStr, CowStrWrapper, FastCowStr, ToCowStr, ToCowStrWrapper};
pub use escaper::Escaper;
pub use render::Render;
pub use unescaped_text::{FastEscape, UnescapedText, UnescapedTextWrapper};
