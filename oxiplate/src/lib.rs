#![doc(issue_tracker_base_url = "https://github.com/0b10011/oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use std::io::{Error, Write};

pub use oxiplate_derive::Oxiplate;

pub mod escapers;

/// Optimized render function trait.
pub trait Render {
    /// Estimated output length of the template.
    const ESTIMATED_LENGTH: usize;

    /// Render the template into a string.
    ///
    /// # Errors
    ///
    /// If strings cannot be written to the formatter.
    fn render(&self) -> Result<String, Error> {
        let mut string = Vec::with_capacity(Self::ESTIMATED_LENGTH);
        self.render_into(&mut string)?;
        unsafe {
            Ok(String::from_utf8_unchecked(string))
        }
    }

    /// Render the template into a writer.
    ///
    /// # Errors
    ///
    /// If strings cannot be written to the formatter.
    fn render_into<W: Write>(&self, writer: &mut W) -> ::std::io::Result<()>;
}
