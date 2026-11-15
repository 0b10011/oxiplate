use core::fmt::{Result, Write};

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
