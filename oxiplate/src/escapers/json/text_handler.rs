use super::{IsEscapedForJson, JsonEscaper};
use crate::{PreparedText, TextHandler};

/// Item that implements `TextHandler` for `JsonEscaper`.
pub struct TextHandlerForJson<'a, T> {
    pub(super) value: &'a T,
}

impl<'a, T> TextHandler<'a, JsonEscaper, T> for &TextHandlerForJson<'a, T>
where
    T: IsEscapedForJson,
{
    fn prepare_text_for_output(&self, escaper: &JsonEscaper) -> PreparedText<'a, T> {
        if <T>::is_escaped(escaper) {
            PreparedText::AlreadyEscaped(self.value)
        } else {
            PreparedText::NeedsEscaped(self.value)
        }
    }
}

impl<'a, T> TextHandler<'a, JsonEscaper, T> for TextHandlerForJson<'a, T> {
    fn prepare_text_for_output(&self, _escaper: &JsonEscaper) -> PreparedText<'a, T> {
        PreparedText::NeedsEscaped(self.value)
    }
}
