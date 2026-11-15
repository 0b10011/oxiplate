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

extern crate alloc;

use core::fmt::{Result, Write};
use core::marker::PhantomData;

use oxiplate_traits::{Escaper, IsEscaped};

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

    type IsEscaped<T> = IsEscapedForHtml<T>;

    #[inline]
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
        match self {
            Self::text => escape_text(f, value),
            Self::attr => escape_attribute_quoted_value(f, value),
            Self::comment => escape_comment_text(f, value),
        }
    }

    fn build_is_escaped<T>(_value: &T) -> Self::IsEscaped<T> {
        IsEscapedForHtml {
            phantom_data: PhantomData,
        }
    }
}

/// Item that implements `IsEscaped` for `HtmlEscaper`.
pub struct IsEscapedForHtml<T> {
    phantom_data: PhantomData<T>,
}

impl<T> IsEscaped<HtmlEscaper> for &IsEscapedForHtml<T>
where
    T: SafeForHtml,
{
    fn is_escaped(&self, escaper: &HtmlEscaper) -> bool {
        <T>::is_safe(escaper)
    }
}

impl<T> IsEscaped<HtmlEscaper> for IsEscapedForHtml<T> {
    fn is_escaped(&self, _escaper: &HtmlEscaper) -> bool {
        false
    }
}

trait SafeForHtml {
    fn is_safe(escaper: &HtmlEscaper) -> bool;
}
impl<T> SafeForHtml for &T
where
    T: SafeForHtml,
{
    fn is_safe(escaper: &HtmlEscaper) -> bool {
        <T>::is_safe(escaper)
    }
}
impl<T> SafeForHtml for &mut T
where
    T: SafeForHtml,
{
    fn is_safe(escaper: &HtmlEscaper) -> bool {
        <T>::is_safe(escaper)
    }
}
impl<T> SafeForHtml for alloc::boxed::Box<T>
where
    T: SafeForHtml,
{
    fn is_safe(escaper: &HtmlEscaper) -> bool {
        <T>::is_safe(escaper)
    }
}
impl<T> SafeForHtml for alloc::rc::Rc<T>
where
    T: SafeForHtml,
{
    fn is_safe(escaper: &HtmlEscaper) -> bool {
        <T>::is_safe(escaper)
    }
}
impl<T> SafeForHtml for alloc::sync::Arc<T>
where
    T: SafeForHtml,
{
    fn is_safe(escaper: &HtmlEscaper) -> bool {
        <T>::is_safe(escaper)
    }
}

macro_rules! numbers_safe_for_html {
    ($($ty:ty)*) => { $(
        impl SafeForHtml for $ty {
            fn is_safe(escaper: &HtmlEscaper) -> bool {
                match escaper {
                    HtmlEscaper::text | HtmlEscaper::attr | HtmlEscaper::comment => true,
                }
            }
        }
    )* };
}
numbers_safe_for_html!(
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
    f32 f64
);

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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::string::String;

    #[test]
    fn test_escape_attribute_quoted_value() {
        let mut string = String::new();
        super::escape_attribute_quoted_value(&mut string, r#"&"' hello"#).unwrap();
        assert_eq!("&amp;&#34;&#39; hello", string);
    }

    #[test]
    fn test_escape_attribute_quoted_value_nothing_to_escape() {
        let mut string = String::new();
        super::escape_attribute_quoted_value(&mut string, r#"Hello world!"#).unwrap();
        assert_eq!("Hello world!", string);
    }
}
