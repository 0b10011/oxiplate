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

use std::borrow::Cow;

/// Escaper group to pass to Oxiplate for JSON escaping.
/// For handling full values instead of just substrings,
/// consider using `raw` in conjuction with something like Serde.
pub enum JsonEscaper {
    /// For escaping a substring that is placed within an existing string.
    /// See [`escape_substring()`] for details.
    Substring,
}

impl super::Escaper for JsonEscaper {
    const DEFAULT: Self = Self::Substring;

    fn escape<'a>(&self, value: &'a str) -> Cow<'a, str> {
        match self {
            Self::Substring => escape_substring(value),
        }
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
#[inline]
#[must_use]
pub fn escape_substring(value: &'_ str) -> Cow<'_, str> {
    value
        .chars()
        .map(|character| match character {
            '"' => r#"\""#.to_string(),
            '\\' => r"\\".to_string(),
            '\u{0000}'..='\u{001F}' => format!("\\u{:04x}", character as u32),
            _ => character.to_string(),
        })
        .collect::<Cow<str>>()
}

#[test]
fn test_escape_substring() {
    assert_eq!(escape_substring(r"\"), r"\\");
    assert_eq!(escape_substring(r#"""#), r#"\""#);
    assert_eq!(escape_substring(r#"\""#), r#"\\\""#);
    assert_eq!(escape_substring("\u{0000}"), r"\u0000");
    assert_eq!(escape_substring("\u{0001}"), r"\u0001");
}
