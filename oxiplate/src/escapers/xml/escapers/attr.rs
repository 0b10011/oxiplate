use core::fmt::{Result, Write};

/// Escape the value as a single- or double-quoted [attribute value](https://www.w3.org/TR/2006/REC-xml11-20060816/#NT-AttValue) in an XML document.
///
/// ```xml.oxip
/// <?xml version="1.1"?>
/// <user id="{{ attr: id }}" />
/// ```
///
/// Encodes `<`, `&`, `'`, and `"` per <https://www.w3.org/TR/2006/REC-xml11-20060816/#NT-AttValue>:
/// > AttValue ::= '"' ([^<&"] | Reference)* '"'
/// >              "'" ([^<&'] | Reference)* "'"
/// 
/// And <https://www.w3.org/TR/2006/REC-xml11-20060816/#syntax>:
/// > To allow attribute values to contain both single and double quotes,
/// > the apostrophe or single-quote character (')
/// > may be represented as "&apos;",
/// > and the double-quote character (") as "&quot;".
///
/// # Errors
///
/// If escaped string cannot be written to the writer.
#[inline]
pub fn escape_attribute_quoted_value<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
    if !value.contains(['<', '&', '"', '\'']) {
        return f.write_str(value);
    }

    for character in value.chars() {
        match character {
            '<' => f.write_str("&lt;")?,
            '&' => f.write_str("&amp;")?,
            '"' => f.write_str("&quot;")?,
            '\'' => f.write_str("&apos;")?,
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
                super::escape_attribute_quoted_value(&mut string, $input).unwrap();
                assert_eq!($expected, string);
            }
        };
    }

    #[test]
    fn test_escape_attribute_quoted_value() {
        let mut string = String::new();
        super::escape_attribute_quoted_value(&mut string, r#"<&"' hello"#).unwrap();
        assert_eq!("&lt;&amp;&quot;&apos; hello", string);
    }

    // Test characters individually to ensure all are checked in the fast path.
    test_escape!(less_than, "<", "&lt;");
    test_escape!(ampersand, "&", "&amp;");
    test_escape!(double_quote, r#"""#, "&quot;");
    test_escape!(single_quote, "'", "&apos;");

    #[test]
    fn test_escape_attribute_quoted_value_nothing_to_escape() {
        let mut string = String::new();
        super::escape_attribute_quoted_value(&mut string, r#"Hello world!"#).unwrap();
        assert_eq!("Hello world!", string);
    }
}
