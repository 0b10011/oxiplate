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

use core::fmt::{Result, Write};
use core::marker::PhantomData;

use oxiplate_traits::{Escaper, IsEscaped};

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

    type IsEscaped<'a, T: 'a> = IsEscapedForJson<'a, T>;

    #[inline]
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
        match self {
            Self::substring => escape_substring(f, value),
        }
    }

    fn build_is_escaped<T>(_value: &T) -> Self::IsEscaped<'_, T> {
        IsEscapedForJson {
            phantom_data: PhantomData,
        }
    }
}

/// Item that implements `IsEscaped` for `JsonEscaper`.
pub struct IsEscapedForJson<'a, T: ?Sized> {
    phantom_data: PhantomData<&'a T>,
}

impl<'a, T> IsEscapedForJson<'a, T> {
    /// Build `IsSafeForJson` for the provided value.
    pub fn new(_value: &'a T) -> Self {
        Self {
            phantom_data: PhantomData,
        }
    }
}

impl IsEscaped for &IsEscapedForJson<'_, usize> {
    type Escaper = JsonEscaper;

    fn is_escaped(&self, escaper: &Self::Escaper) -> bool {
        match escaper {
            JsonEscaper::substring => true,
        }
    }
}

impl<T> IsEscaped for IsEscapedForJson<'_, T> {
    type Escaper = JsonEscaper;

    fn is_escaped(&self, _escaper: &Self::Escaper) -> bool {
        false
    }
}

/// Escape the value as part of a [string](https://www.rfc-editor.org/rfc/rfc8259#section-7) in a JSON document.
///
/// ```json.oxip
/// {"string": "A {{ type }}"}
/// ```
///
/// Escapes each quotation mark (U+0022) and reverse solidus (U+005C),
/// and all control characters (U+0000 through U+001F)
/// by prefixing them with a reverse solidus (U+005C).
///
/// # Errors
///
/// If escaped string cannot be written to the writer.
#[inline]
pub fn escape_substring<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
    for character in value.chars() {
        match character {
            '"' => f.write_str(r#"\""#)?,
            '\\' => f.write_str(r"\\")?,
            '\u{0000}'..='\u{001F}' => write!(f, "\\u{:04x}", character as u32)?,
            _ => f.write_char(character)?,
        }
    }

    Ok(())
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::string::String;

    #[test]
    fn test_escape_substring() {
        fn escape(raw: &str) -> String {
            let mut escaped = String::with_capacity(raw.len());
            super::escape_substring(&mut escaped, raw).unwrap();
            escaped
        }
        assert_eq!(escape(r"\"), r"\\");
        assert_eq!(escape(r#"""#), r#"\""#);
        assert_eq!(escape(r#"\""#), r#"\\\""#);
        assert_eq!(escape("\u{0000}"), r"\u0000");
        assert_eq!(escape("\u{0001}"), r"\u0001");
    }
}
