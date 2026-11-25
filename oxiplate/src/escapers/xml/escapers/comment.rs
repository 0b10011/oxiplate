use core::fmt::{Result, Write};

/// Escape the value as [comment text](https://www.w3.org/TR/2006/REC-xml11-20060816/#sec-comments) in an XML document.
///
/// ```xml.oxip
/// <?xml version="1.1"?>
/// <!-- {{ comment: user_text }} -->
/// ```
///
/// Replaces `-` with visually similar characters that aren't parsed specially
/// when specific patterns of those characters that are disallowed are found.
///
/// Per <https://www.w3.org/TR/2006/REC-xml11-20060816/#sec-comments>:
/// > For compatibility, the string "--" (double-hyphen) MUST NOT occur within comments.
/// > [...]
/// > [15] Comment ::= '<!--' ((Char - '-') | ('-' (Char - '-')))* '-->'
/// > [...]
/// > Note that the grammar does not allow a comment ending in --->.
///
/// # Errors
///
/// If escaped string cannot be written to the writer.
#[inline]
pub fn escape_comment_text<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
    if
        // Cannot start with `-` to avoid double hyphens
        // if the previously output character happened to be `-`.
        !value.starts_with('-')

        // Cannot contain `--` to avoid double hyphens.
        && !value.contains("--")

        // Cannot end with `-` to avoid double hyphens
        // if the next output character happened to be `-`,
        // or if the comment ends after this with `-->`.
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
            _ => f.write_char(character)?,
        }
    }

    Ok(())
}
