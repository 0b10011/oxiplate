use oxiplate_traits::TextHandler;

use super::{IsEscapedForJson, JsonEscaper};

/// Item that implements `TextHandler` for `JsonEscaper`.
pub struct TextHandlerForJson<'a, T> {
    pub(super) value: &'a T,
}

impl<'a, T> TextHandler<'a, JsonEscaper, T> for &TextHandlerForJson<'a, T>
where
    T: IsEscapedForJson,
{
    fn prepare_text_for_output(&self, escaper: &JsonEscaper) -> (&'a T, bool) {
        (self.value, <T>::is_escaped(escaper))
    }
}

impl<'a, T> TextHandler<'a, JsonEscaper, T> for TextHandlerForJson<'a, T> {
    fn prepare_text_for_output(&self, _escaper: &JsonEscaper) -> (&'a T, bool) {
        (self.value, false)
    }
}
