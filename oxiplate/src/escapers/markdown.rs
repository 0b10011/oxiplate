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

#[cfg(test)]
extern crate alloc;

#[cfg(test)]
use alloc::format;
#[cfg(test)]
use alloc::string::String;
use core::fmt::{Result, Write};

use oxiplate_traits::Escaper;

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
///
/// # Errors
///
/// If escaped string cannot be written to the writer.
#[inline]
pub fn escape_unformatted_text<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
    let mut start_of_string = true;
    let mut needs_whitespace = false;
    macro_rules! append {
        ($($character:expr),+) => {
            {
                if start_of_string {
                    start_of_string = false;
                    needs_whitespace = false;
                } else if needs_whitespace {
                    f.write_char(' ')?;
                    needs_whitespace = false;
                }
                $(f.write_char($character)?;)+
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

    Ok(())
}

#[test]
fn null() {
    let mut string = String::new();
    escape_unformatted_text(&mut string, "\u{0000}").unwrap();
    assert_eq!("\u{FFFD}", string);
}

#[test]
fn ascii_puncutation_characters() {
    let mut string = String::new();
    escape_unformatted_text(&mut string, r#"An ASCII punctuation character is !, ", #, $, %, &, ', (, ), *, +, ,, -, ., / (U+0021–2F), :, ;, <, =, >, ?, @ (U+003A–0040), [, \, ], ^, _, ` (U+005B–0060), {, |, }, or ~ (U+007B–007E)."#).unwrap();

    // All ASCII punctuation characters should be prefixed by `\`.
    assert_eq!(
        r#"An ASCII punctuation character is \!\, \"\, \#\, \$\, \%\, \&\, \'\, \(\, \)\, \*\, \+\, \,\, \-\, \.\, \/ \(U\+0021–2F\)\, \:\, \;\, \<\, \=\, \>\, \?\, \@ \(U\+003A–0040\)\, \[\, \\\, \]\, \^\, \_\, \` \(U\+005B–0060\)\, \{\, \|\, \}\, or \~ \(U\+007B–007E\)\."#,
        string
    );
}

#[cfg(test)]
static WHITESPACE: &str =
    "\u{0009}\u{000A}\u{000C}\u{000D}\u{0020}\u{00A0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003} \
     \u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200A}\u{202F}\u{205F}\u{3000}";

#[test]
fn whitespace_only() {
    let mut string = String::new();
    escape_unformatted_text(&mut string, WHITESPACE).unwrap();
    // Whitespace should be trimmed, leaving an empty string.
    assert_eq!("", string);
}

#[test]
fn whitespace_prefix() {
    let mut string = String::new();
    escape_unformatted_text(&mut string, &format!("{}world", WHITESPACE)).unwrap();
    // Whitespace should be trimmed, leaving just non-whitespace characters.
    assert_eq!("world", string);
}

#[test]
fn whitespace_suffix() {
    let mut string = String::new();
    escape_unformatted_text(&mut string, &format!("hello{}", WHITESPACE)).unwrap();
    // Whitespace should be trimmed, leaving just non-whitespace characters.
    assert_eq!("hello", string);
}

#[test]
fn whitespace() {
    let mut string = String::new();
    escape_unformatted_text(&mut string, &format!("hello{}world", WHITESPACE)).unwrap();
    // Whitespace between non-whitespace characters should be collapsed into a single space.
    assert_eq!("hello world", string);
}
