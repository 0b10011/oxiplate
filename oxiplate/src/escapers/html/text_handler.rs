use super::{HtmlEscaper, IsEscapedForHtml};
use crate::{PreparedText, TextHandler};

/// Item that implements `TextHandler` for `HtmlEscaper`.
pub struct TextHandlerForHtml<'a, T> {
    pub(super) value: &'a T,
}

impl<'a, T> TextHandler<'a, HtmlEscaper, T> for &TextHandlerForHtml<'a, T>
where
    T: IsEscapedForHtml,
{
    fn prepare_text_for_output(&self, escaper: &HtmlEscaper) -> PreparedText<'a, T> {
        if <T>::is_escaped(escaper) {
            PreparedText::AlreadyEscaped(self.value)
        } else {
            PreparedText::NeedsEscaped(self.value)
        }
    }
}

impl<'a, T> TextHandler<'a, HtmlEscaper, T> for TextHandlerForHtml<'a, T> {
    fn prepare_text_for_output(&self, _escaper: &HtmlEscaper) -> PreparedText<'a, T> {
        PreparedText::NeedsEscaped(self.value)
    }
}
