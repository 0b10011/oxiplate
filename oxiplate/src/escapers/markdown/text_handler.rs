use super::{IsEscapedForMarkdown, MarkdownEscaper};
use crate::{PreparedText, TextHandler};

/// Item that implements `TextHandler` for `MarkdownEscaper`.
pub struct TextHandlerForMarkdown<'a, T> {
    pub(super) value: &'a T,
}

impl<'a, T> TextHandler<'a, MarkdownEscaper, T> for &TextHandlerForMarkdown<'a, T>
where
    T: IsEscapedForMarkdown,
{
    fn prepare_text_for_output(&self, escaper: &MarkdownEscaper) -> PreparedText<'a, T> {
        if <T>::is_escaped(escaper) {
            PreparedText::AlreadyEscaped(self.value)
        } else {
            PreparedText::NeedsEscaped(self.value)
        }
    }
}

impl<'a, T> TextHandler<'a, MarkdownEscaper, T> for TextHandlerForMarkdown<'a, T> {
    fn prepare_text_for_output(&self, _escaper: &MarkdownEscaper) -> PreparedText<'a, T> {
        PreparedText::NeedsEscaped(self.value)
    }
}
