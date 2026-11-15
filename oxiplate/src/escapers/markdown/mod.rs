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
mod is_escaped;
mod text_handler;

extern crate alloc;

use core::fmt::{Result, Write};

use oxiplate_traits::Escaper;

pub use self::escapers::escape_unformatted_text;
pub use self::is_escaped::IsEscapedForMarkdown;
use self::text_handler::TextHandlerForMarkdown;

/// Escaper group to pass to Oxiplate for Markdown escaping.
#[allow(non_camel_case_types)]
pub enum MarkdownEscaper {
    /// Escaper for text in a Markdown document.
    /// See [`escape_unformatted_text()`] for details.
    text,
}

impl Escaper for MarkdownEscaper {
    const DEFAULT: Self = Self::text;

    type TextHandler<'a, T: 'a> = TextHandlerForMarkdown<'a, T>;

    #[inline]
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
        match self {
            Self::text => escape_unformatted_text(f, value),
        }
    }

    fn build_text_handler<T>(value: &T) -> Self::TextHandler<'_, T> {
        TextHandlerForMarkdown { value }
    }
}
