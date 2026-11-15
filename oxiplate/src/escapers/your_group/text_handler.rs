use oxiplate_traits::TextHandler;

use super::YourEscaper;
use super::is_escaped::IsEscapedForYourGroup;

/// Item that implements `TextHandler` for `YourEscaper`.
pub struct TextHandlerForYourGroup<'a, T> {
    pub(super) value: &'a T,
}

impl<'a, T> TextHandler<'a, YourEscaper, T> for &TextHandlerForYourGroup<'a, T>
where
    T: IsEscapedForYourGroup,
{
    fn prepare_text_for_output(&self, escaper: &YourEscaper) -> (&'a T, bool) {
        (self.value, <T>::is_escaped(escaper))
    }
}

impl<'a, T> TextHandler<'a, YourEscaper, T> for TextHandlerForYourGroup<'a, T> {
    fn prepare_text_for_output(&self, _escaper: &YourEscaper) -> (&'a T, bool) {
        (self.value, false)
    }
}
