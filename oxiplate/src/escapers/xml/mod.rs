//! Built-in XML escapers.
//!
//! To use the defined escaper,
//! add the following to `/oxiplate.toml`:
//!
//! ```toml
//! [escaper_groups.xml]
//! escaper = "::oxiplate::escapers::xml::XmlEscaper"
//! ```
//!
//! Escaper functions are public in case you want to reuse them in your own escaper group.

mod escapers;
mod is_escaped;
mod text_handler;

use core::fmt::{Result, Write};

pub use self::escapers::attr::escape_attribute_quoted_value;
pub use self::escapers::comment::escape_comment_text;
pub use self::escapers::text::escape_text;
pub use self::is_escaped::IsEscapedForXml;
use self::text_handler::TextHandlerForXml;
use crate::Escaper;

/// Escaper group to pass to Oxiplate for XML escaping.
#[allow(non_camel_case_types)]
pub enum XmlEscaper {
    /// Escaper for [text](https://www.w3.org/TR/2006/REC-xml11-20060816/#syntax) in an XML document.
    /// See [`escape_text()`] for details.
    text,

    /// Escaper for single- and double-quoted [attribute values](https://xml.spec.whatwg.org/#syntax-attribute-value) in an XML document.
    /// See [`escape_attribute_quoted_value()`] for details.
    attr,

    /// Escaper for [comment text](https://xml.spec.whatwg.org/#comments) in an XML document.
    /// See [`escape_comment_text()`] for details.
    comment,
}

impl Escaper for XmlEscaper {
    const DEFAULT: Self = Self::text;

    type TextHandler<'a, T: 'a> = TextHandlerForXml<'a, T>;

    #[inline]
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
        match self {
            Self::text => escape_text(f, value),
            Self::attr => escape_attribute_quoted_value(f, value),
            Self::comment => escape_comment_text(f, value),
        }
    }

    fn build_text_handler<T>(value: &T) -> Self::TextHandler<'_, T> {
        TextHandlerForXml { value }
    }
}
