#![no_std]
#![doc(issue_tracker_base_url = "https://github.com/0b10011/oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod cow_str;
mod escaper;
mod render;
mod unescaped_text;

pub use self::cow_str::{CowStr, FastCowStr};
pub use self::escaper::{Escaper, PreparedText, TextHandler};
pub use self::render::Render;
pub use self::unescaped_text::{FastWriteEscaped, FastWriteRaw};

/// Items only meant to be used by `oxiplate-derive` in code generation.
pub mod private {
    pub use super::cow_str::{CowStrWrapper, ToCowStr, ToCowStrWrapper};
    pub use super::unescaped_text::{UnescapedTextWrapper, WriteEscaped, WriteRaw};
}
