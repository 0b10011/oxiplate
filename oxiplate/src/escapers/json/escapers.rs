use core::fmt::{Result, Write};

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
