use core::fmt::{Result, Write};

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
