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

use std::fmt::{Result, Write};

/// Escaper group to pass to Oxiplate for HTML escaping.
pub enum HtmlEscaper {
    /// Escaper for [text](https://html.spec.whatwg.org/#text-content) in an HTML document.
    /// See [`escape_text()`] for details.
    Text,

    /// Escaper for single- and double-quoted [attribute values](https://html.spec.whatwg.org/#syntax-attribute-value) in an HTML document.
    /// See [`escape_attribute_quoted_value()`] for details.
    Attr,

    /// Escaper for [comment text](https://html.spec.whatwg.org/#comments) in an HTML document.
    /// See [`escape_comment_text()`] for details.
    Comment,
}

impl super::Escaper for HtmlEscaper {
    const DEFAULT: Self = Self::Text;

    #[inline]
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
        match self {
            Self::Text => escape_text(f, value),
            Self::Attr => escape_attribute_quoted_value(f, value),
            Self::Comment => escape_comment_text(f, value),
        }
    }
}

/// Escape the value as [text](https://html.spec.whatwg.org/#text-content) in an HTML document.
///
/// ```html.oxip
/// <!DOCTYPE html>
/// <h1>{{ text: title }}</h1>
/// ```
///
/// Encodes `&` and `<` per <https://html.spec.whatwg.org/#elements-2>:
/// > Normal elements can have text, character references, other elements, and comments,
/// > but the text must not contain the character U+003C LESS-THAN SIGN (`<`) or an ambiguous ampersand.
///
/// The shortest encodings for each were selected,
/// rather than a specific encoding style,
/// to reduce the length of the final template.
///
/// # Errors
///
/// If escaped string cannot be written to the writer.
#[inline]
pub fn escape_text<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
    if !value.contains(['&', '<']) {
        return f.write_str(value);
    }

    for character in value.chars() {
        match character {
            '&' => f.write_str("&amp;")?,
            '<' => f.write_str("&lt;")?,
            _ => f.write_char(character)?,
        }
    }

    Ok(())
}

/// Escape the value as a single- or double-quoted [attribute value](https://html.spec.whatwg.org/#syntax-attribute-value) in an HTML document.
///
/// ```html.oxip
/// <!DOCTYPE html>
/// <a href="/user/{{ attr: user_id }}">Profile</a>
/// ```
///
/// Encodes `&`, `'`, and `"` per <https://html.spec.whatwg.org/#attributes-2>:
/// > Attribute values are a mixture of text and character references,
/// > except with the additional restriction that the text cannot contain an ambiguous ampersand.
/// >
/// > [...]
/// >
/// > Single-quoted attribute value syntax
/// > [...] the attribute value [...] must not contain any literal U+0027 APOSTROPHE characters (`'`) [...]
/// >
/// > [...]
/// >
/// > Double-quoted attribute value syntax
/// > [...] the attribute value [...] must not contain any literal U+0022 QUOTATION MARK characters (`"`) [...]
///
/// The shortest encodings for each were selected,
/// rather than a specific encoding style,
/// to reduce the length of the final template.
///
/// # Errors
///
/// If escaped string cannot be written to the writer.
#[inline]
pub fn escape_attribute_quoted_value<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
    if !value.contains(['&', '"', '\'']) {
        return f.write_str(value);
    }

    for character in value.chars() {
        match character {
            '&' => f.write_str("&amp;")?,
            '"' => f.write_str("&#34;")?,
            '\'' => f.write_str("&#39;")?,
            _ => f.write_char(character)?,
        }
    }

    Ok(())
}

/// Escape the value as [comment text](https://html.spec.whatwg.org/#comments) in an HTML document.
///
/// ```html.oxip
/// <!DOCTYPE html>
/// <!-- {{ comment: user_text }} -->
/// ```
///
/// Replaces `-`, `!`, `<`, and `>` with visually similar characters that aren't parsed specially
/// when specific patterns of those characters that are disallowed are found.
///
/// Per <https://html.spec.whatwg.org/#comments>:
/// > Optionally, text, with the additional restriction
/// > that the text must not start with the string `>`,
/// > nor start with the string `->`,
/// > nor contain the strings `<!--`, `-->`, or `--!>`,
/// > nor end with the string `<!-`.
///
/// XML 1.0 also does not allow two consecutive hyphens in a comment.
/// Per <https://www.w3.org/TR/REC-xml/#sec-comments>:
/// > For compatibility,
/// > the string " -- " (double-hyphen) MUST NOT occur within comments.
///
/// # Errors
///
/// If escaped string cannot be written to the writer.
#[inline]
pub fn escape_comment_text<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
    if
    // Cannot start with `>` for HTML
    !value.starts_with('>')

        // Cannot start with `->` for HTML
        // Cannot start with `-` to avoid double hyphens for XML 1.0
        && !value.starts_with('-')

        // Cannot contain `<!--`, `-->` or `--!>` for HTML
        // Cannot contain `--` to avoid double hyphens for XML 1.0
        && !value.contains("--")

        // Cannot end with `<!-` for HTML
        && !value.ends_with("<!-")

        // Cannot end with `-` to double hyphens for XML 1.0
        && !value.ends_with('-')
    {
        return f.write_str(value);
    }

    // If any disallowed substrings are found,
    // replace all of the characters that could have been involved
    // to ensure all offenders are replaced
    // and to possibly speed up replacement.
    for character in value.chars() {
        match character {
            '-' => f.write_char('−')?,
            '!' => f.write_char('ǃ')?,
            '<' => f.write_char('‹')?,
            '>' => f.write_char('›')?,
            _ => f.write_char(character)?,
        }
    }

    Ok(())
}
