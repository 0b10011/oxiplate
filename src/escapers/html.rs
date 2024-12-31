//! Built-in HTML escapers.
//!
//! To use the defined escaper,
//! add the following to `/oxiplate.toml`:
//!
//! ```toml
//! [escaper_groups.html]
//! escaper = "::oxiplate::HtmlEscaper"
//! ```
//!
//! Escaper functions are public in case you want to reuse them in your own escaper group.

use std::borrow::Cow;

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
    fn escape<'a>(&self, value: &'a str) -> Cow<'a, str> {
        match self {
            Self::Text => escape_text(value),
            Self::Attr => escape_attribute_quoted_value(value),
            Self::Comment => escape_comment_text(value),
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
#[inline]
#[must_use]
pub fn escape_text(value: &str) -> Cow<str> {
    if !value.contains(['&', '<']) {
        return Cow::Borrowed(value);
    }

    value
        .chars()
        .map(|character| match character {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            _ => character.to_string(),
        })
        .collect::<Cow<str>>()
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
#[inline]
#[must_use]
pub fn escape_attribute_quoted_value(value: &str) -> Cow<str> {
    if !value.contains(['&', '<', '"', '\'']) {
        return Cow::Borrowed(value);
    }

    value
        .chars()
        .map(|character| match character {
            '&' => "&amp;".to_string(),
            '"' => "&#34;".to_string(),
            '\'' => "&#39;".to_string(),
            _ => character.to_string(),
        })
        .collect::<Cow<str>>()
}

/// Escape the value as [comment text](https://html.spec.whatwg.org/#comments) in an HTML document.
///
/// ```html.oxip
/// <!DOCTYPE html>
/// <!-- {{ comment: user_text }} -->
/// ```
///
/// Replaces `-`, `<`, and `>` with visually similar characters that aren't parsed specially.
/// Per <https://html.spec.whatwg.org/#comments>:
/// > Optionally, text, with the additional restriction
/// > that the text must not start with the string `>`,
/// > nor start with the string `->`,
/// > nor contain the strings `<!--`, `-->`, or `--!>`,
/// > nor end with the string `<!-`.
///
/// The shortest encodings for each were selected,
/// rather than a specific encoding style,
/// to reduce the length of the final template.
#[inline]
#[must_use]
pub fn escape_comment_text(value: &str) -> Cow<str> {
    if !value.contains(['-', '<', '>']) {
        return Cow::Borrowed(value);
    }

    value
        .chars()
        .map(|character| match character {
            '-' => '−'.to_string(),
            '<' => '‹'.to_string(),
            '>' => '›'.to_string(),
            _ => character.to_string(),
        })
        .collect::<Cow<str>>()
}
