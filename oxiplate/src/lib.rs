#![doc(issue_tracker_base_url = "https://github.com/0b10011/oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use std::fmt::Write;

pub use oxiplate_derive::Oxiplate;

pub mod escapers;

/// Optimized render function trait.
pub trait Render {
    /// Optimized render function.
    ///
    /// # Errors
    ///
    /// If strings cannot be written to the formatter.
    fn render<W: Write>(&self, writer: &mut W) -> ::std::fmt::Result;
}
