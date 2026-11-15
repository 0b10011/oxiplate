use oxiplate_traits::TextHandler;

use super::{HtmlEscaper, IsEscapedForHtml};

/// Item that implements `TextHandler` for `HtmlEscaper`.
pub struct TextHandlerForHtml<'a, T> {
    pub(super) value: &'a T,
}

impl<'a, T> TextHandler<'a, HtmlEscaper, T> for &TextHandlerForHtml<'a, T>
where
    T: IsEscapedForHtml,
{
    fn prepare_text_for_output(&self, escaper: &HtmlEscaper) -> (&'a T, bool) {
        (self.value, <T>::is_escaped(escaper))
    }
}

impl<'a, T> TextHandler<'a, HtmlEscaper, T> for TextHandlerForHtml<'a, T> {
    fn prepare_text_for_output(&self, _escaper: &HtmlEscaper) -> (&'a T, bool) {
        (self.value, false)
    }
}
