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
    const DEFAULT: Self = Self::Text;

    #[inline]
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
pub fn escape_text(value: &'_ str) -> Cow<'_, str> {
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
pub fn escape_attribute_quoted_value(value: &'_ str) -> Cow<'_, str> {
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
#[inline]
#[must_use]
pub fn escape_comment_text(value: &'_ str) -> Cow<'_, str> {
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
        return Cow::Borrowed(value);
    }

    // If any disallowed substrings are found,
    // replace all of the characters that could have been involved
    // to ensure all offenders are replaced
    // and to possibly speed up replacement.
    value
        .chars()
        .map(|character| match character {
            '-' => '−'.to_string(),
            '!' => 'ǃ'.to_string(),
            '<' => '‹'.to_string(),
            '>' => '›'.to_string(),
            _ => character.to_string(),
        })
        .collect::<Cow<str>>()
}
