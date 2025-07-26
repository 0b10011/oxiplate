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

use std::borrow::Cow;

/// Escaper group to pass to Oxiplate for Markdown escaping.
pub enum MarkdownEscaper {
    /// Escaper for text in a Markdown document.
    /// See [`escape_unformatted_text()`] for details.
    Text,
}

impl super::Escaper for MarkdownEscaper {
    const DEFAULT: Self = Self::Text;

    #[inline]
    fn escape<'a>(&self, value: &'a str) -> Cow<'a, str> {
        match self {
            Self::Text => escape_unformatted_text(value),
        }
    }
}

/// Escape the value as [unformatted text](https://spec.commonmark.org/0.31.2/#textual-content) in an Markdown document.
///
/// ```md.oxip
/// These are all equivalent if this escaper is set as the default and named "md":
/// - {{ name }}
/// - {{ text: name }}
/// - {{ md.text: name }}
/// ```
///
/// [Escapes][] all [ASCII punctuation characters][],
/// trims [Unicode whitespace characters][] from the beginning and end of the string,
/// collapses all other Unicode whitespace into a single Space (U+0020),
/// and replaces NULL (U+0000) with the REPLACEMENT CHARACTER (U+FFFD).
///
/// [Escapes]: https://spec.commonmark.org/0.31.2/#backslash-escapes
/// [ASCII punctuation characters]: https://spec.commonmark.org/0.31.2/#ascii-punctuation-character
/// [Unicode whitespace characters]: https://spec.commonmark.org/0.31.2/#unicode-whitespace-character
#[inline]
#[must_use]
pub fn escape_unformatted_text(value: &'_ str) -> Cow<'_, str> {
    let mut string = String::with_capacity(value.len());
    let mut start_of_string = true;
    let mut needs_whitespace = false;
    macro_rules! append {
        ($($character:expr),+) => {
            {
                if start_of_string {
                    start_of_string = false;
                    needs_whitespace = false;
                } else if needs_whitespace {
                    string.push(' ');
                    needs_whitespace = false;
                }
                $(string.push($character);)+
            }
        };
    }
    for character in value.chars() {
        match character {
            // Per https://spec.commonmark.org/0.31.2/#insecure-characters:
            // > For security reasons,
            // > the Unicode character U+0000 must be replaced with the REPLACEMENT CHARACTER (U+FFFD).
            '\u{0000}' => append!('\u{FFFD}'),

            // Per https://spec.commonmark.org/0.31.2/#backslash-escapes:
            // > Any ASCII punctuation character may be backslash-escaped
            // and:
            // > Backslashes before other characters are treated as literal backslashes
            // and from https://spec.commonmark.org/0.31.2/#ascii-punctuation-character:
            // > An ASCII punctuation character is
            // > !, ", #, $, %, &, ', (, ), *, +, ,, -, ., / (U+0021–2F),
            // > :, ;, <, =, >, ?, @ (U+003A–0040),
            // > [, \, ], ^, _, ` (U+005B–0060),
            // > {, |, }, or ~ (U+007B–007E).
            '\u{0021}'..='\u{002F}'
            | '\u{003A}'..='\u{0040}'
            | '\u{005B}'..='\u{0060}'
            | '\u{007B}'..='\u{007E}' => {
                append!('\\', character);
            }

            // [Unicode whitespace characters](https://spec.commonmark.org/0.31.2/#unicode-whitespace-character).
            // In order: tab, line feed, form feed, carriage return, and Unicode Zs general category.
            // Replace one or more with a single space to avoid unintended formatting changes.
            '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{000D}' | '\u{0020}' | '\u{00A0}'
            | '\u{1680}' | '\u{2000}' | '\u{2001}' | '\u{2002}' | '\u{2003}' | '\u{2004}'
            | '\u{2005}' | '\u{2006}' | '\u{2007}' | '\u{2008}' | '\u{2009}' | '\u{200A}'
            | '\u{202F}' | '\u{205F}' | '\u{3000}' => {
                needs_whitespace = true;
            }

            _ => append!(character),
        }
    }

    string.into()
}
