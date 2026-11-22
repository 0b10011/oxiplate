extern crate alloc;

use super::MarkdownEscaper;
use crate::is_escaped;

is_escaped!(pub IsEscapedForMarkdown, MarkdownEscaper);

impl IsEscapedForMarkdown for bool {
    #[inline]
    fn is_escaped(escaper: &MarkdownEscaper) -> bool {
        match escaper {
            MarkdownEscaper::text => true,
        }
    }
}
