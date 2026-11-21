//! Built-in JSON escapers.
//!
//! To use the defined escaper,
//! add the following to `/oxiplate.toml`:
//!
//! ```toml
//! escaper_groups.json.escaper = "::oxiplate::escapers::json::JsonEscaper"
//! ```
//!
//! Escaper functions are public in case you want to reuse them in your own escaper group.

mod escapers;
mod is_escaped;
mod text_handler;

use core::fmt::{Result, Write};

pub use self::escapers::escape_substring;
pub use self::is_escaped::IsEscapedForJson;
use self::text_handler::TextHandlerForJson;
use crate::Escaper;

/// Escaper group to pass to Oxiplate for JSON escaping.
/// For handling full values instead of just substrings,
/// consider using `raw` in conjuction with something like Serde.
#[allow(non_camel_case_types)]
pub enum JsonEscaper {
    /// For escaping a substring that is placed within an existing string.
    /// See [`escape_substring()`] for details.
    substring,
}

impl Escaper for JsonEscaper {
    const DEFAULT: Self = Self::substring;

    type TextHandler<'a, T: 'a> = TextHandlerForJson<'a, T>;

    #[inline]
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
        match self {
            Self::substring => escape_substring(f, value),
        }
    }

    fn build_text_handler<T>(value: &T) -> Self::TextHandler<'_, T> {
        TextHandlerForJson { value }
    }
}
