use core::fmt::{Result, Write};

/// Escape the value as [text](https://www.w3.org/TR/2006/REC-xml11-20060816/#syntax) in an XML document.
///
/// ```xml.oxip
/// <?xml version="1.1"?>
/// <user>{{ text: user.name }}</user>
/// ```
///
/// Encodes `&`, `<`, and `>` per <https://www.w3.org/TR/2006/REC-xml11-20060816/#syntax>:
/// > The ampersand character (&) and the left angle bracket (<)
/// > MUST NOT appear in their literal form,
/// > except when used as markup delimiters,
/// > or within a comment,
/// > a processing instruction,
/// > or a CDATA section.
/// > If they are needed elsewhere,
/// > they MUST be escaped using either numeric character references
/// > or the strings "&amp;" and "&lt;" respectively.
/// > The right angle bracket (>)
/// > may be represented using the string "&gt;",
/// > and MUST, for compatibility,
/// > be escaped using either "&gt;"
/// > or a character reference
/// > when it appears in the string "]]>" in content,
/// > when that string is not marking the end of a CDATA section.
///
/// While `>` only needs to be encoded when present in `]]>`,
/// because `]` is more common in prose
/// and `>` should be fairly rare,
/// this currently always escapes `>`
/// to additional character matching
/// or a buffer of preceding characters.
///
/// # Errors
///
/// If escaped string cannot be written to the writer.
#[inline]
pub fn escape_text<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
    if !value.contains(['&', '<', '>']) {
        return f.write_str(value);
    }

    for character in value.chars() {
        match character {
            '&' => f.write_str("&amp;")?,
            '<' => f.write_str("&lt;")?,
            '>' => f.write_str("&gt;")?,
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

    macro_rules! test_escape {
        ($fn_name:ident, $input:literal, $expected:literal) => {
            #[test]
            fn $fn_name() {
                let mut string = String::new();
                super::escape_text(&mut string, $input).unwrap();
                assert_eq!($expected, string);
            }
        };
    }

    // Test characters individually to ensure all are checked in the fast path.
    test_escape!(ampersand, "&", "&amp;");
    test_escape!(less_than, "<", "&lt;");
    test_escape!(cdata_end, "]]>", "]]&gt;");
}
