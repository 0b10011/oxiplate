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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use oxiplate_traits::PreparedText;

    use super::{IsEscapedForJson, JsonEscaper, TextHandlerForJson};
    use crate::TextHandler as _;

    #[derive(PartialEq, Eq)]
    struct NotEscapedForJson;

    impl IsEscapedForJson for NotEscapedForJson {
        fn is_escaped(_escaper: &JsonEscaper) -> bool {
            false
        }
    }

    #[test]
    fn not_escaped_for_json() {
        let PreparedText::NeedsEscaped(&NotEscapedForJson) = (&&TextHandlerForJson {
            value: &NotEscapedForJson,
        })
            .prepare_text_for_output(&JsonEscaper::substring)
        else {
            panic!("Expected value to need escaping");
        };
    }
}
