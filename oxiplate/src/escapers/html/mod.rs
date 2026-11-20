//! Built-in HTML escapers.
//!
//! To use the defined escaper,
//! add the following to `/oxiplate.toml`:
//!
//! ```toml
//! [escaper_groups.html]
//! escaper = "::oxiplate::escapers::html::HtmlEscaper"
//! ```
//!
//! Escaper functions are public in case you want to reuse them in your own escaper group.

mod escapers;

use core::fmt::{Result, Write};

pub use self::escapers::attr::escape_attribute_quoted_value;
pub use self::escapers::comment::escape_comment_text;
pub use self::escapers::text::escape_text;
use crate::Escaper;

/// Escaper group to pass to Oxiplate for HTML escaping.
#[allow(non_camel_case_types)]
pub enum HtmlEscaper {
    /// Escaper for [text](https://html.spec.whatwg.org/#text-content) in an HTML document.
    /// See [`escape_text()`] for details.
    text,

    /// Escaper for single- and double-quoted [attribute values](https://html.spec.whatwg.org/#syntax-attribute-value) in an HTML document.
    /// See [`escape_attribute_quoted_value()`] for details.
    attr,

    /// Escaper for [comment text](https://html.spec.whatwg.org/#comments) in an HTML document.
    /// See [`escape_comment_text()`] for details.
    comment,
}

impl Escaper for HtmlEscaper {
    const DEFAULT: Self = Self::text;

    #[inline]
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
        match self {
            Self::text => escape_text(f, value),
            Self::attr => escape_attribute_quoted_value(f, value),
            Self::comment => escape_comment_text(f, value),
        }
    }
}
