use core::fmt::{Result, Write};

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
