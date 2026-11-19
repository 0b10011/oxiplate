use super::YourEscaper;
use super::is_escaped::IsEscapedForYourGroup;
use crate::{PreparedText, TextHandler};

/// Item that implements `TextHandler` for `YourEscaper`.
pub struct TextHandlerForYourGroup<'a, T> {
    pub(super) value: &'a T,
}

impl<'a, T> TextHandler<'a, YourEscaper, T> for &TextHandlerForYourGroup<'a, T>
where
    T: IsEscapedForYourGroup,
{
    fn prepare_text_for_output(&self, escaper: &YourEscaper) -> PreparedText<'a, T> {
        if <T>::is_escaped(escaper) {
            PreparedText::AlreadyEscaped(self.value)
        } else {
            PreparedText::NeedsEscaped(self.value)
        }
    }
}

impl<'a, T> TextHandler<'a, YourEscaper, T> for TextHandlerForYourGroup<'a, T> {
    fn prepare_text_for_output(&self, _escaper: &YourEscaper) -> PreparedText<'a, T> {
        PreparedText::NeedsEscaped(self.value)
    }
}
