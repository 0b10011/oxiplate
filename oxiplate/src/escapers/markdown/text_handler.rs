use oxiplate_traits::TextHandler;

use super::{IsEscapedForMarkdown, MarkdownEscaper};

/// Item that implements `TextHandler` for `MarkdownEscaper`.
pub struct TextHandlerForMarkdown<'a, T> {
    pub(super) value: &'a T,
}

impl<'a, T> TextHandler<'a, MarkdownEscaper, T> for &TextHandlerForMarkdown<'a, T>
where
    T: IsEscapedForMarkdown,
{
    fn prepare_text_for_output(&self, escaper: &MarkdownEscaper) -> (&'a T, bool) {
        (self.value, <T>::is_escaped(escaper))
    }
}

impl<'a, T> TextHandler<'a, MarkdownEscaper, T> for TextHandlerForMarkdown<'a, T> {
    fn prepare_text_for_output(&self, _escaper: &MarkdownEscaper) -> (&'a T, bool) {
        (self.value, false)
    }
}
