//! Built-in Markdown ([CommonMark](https://commonmark.org/)) escapers.
//!
//! To use the defined escaper,
//! add the following to `/oxiplate.toml`:
//!
//! ```toml
//! escaper_groups.md.escaper = "::oxiplate::escapers::markdown::MarkdownEscaper"
//! ```
//!
//! Escaper functions are public in case you want to reuse them in your own escaper group.

mod escapers;

use core::fmt::{Result, Write};

pub use self::escapers::escape_unformatted_text;
use crate::Escaper;

/// Escaper group to pass to Oxiplate for Markdown escaping.
#[allow(non_camel_case_types)]
pub enum MarkdownEscaper {
    /// Escaper for text in a Markdown document.
    /// See [`escape_unformatted_text()`] for details.
    text,
}

impl Escaper for MarkdownEscaper {
    const DEFAULT: Self = Self::text;

    #[inline]
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
        match self {
            Self::text => escape_unformatted_text(f, value),
        }
    }
}
