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
    #[inline]
    fn prepare_text_for_output(&self, escaper: &MarkdownEscaper) -> PreparedText<'a, T> {
        if <T>::is_escaped(escaper) {
            PreparedText::AlreadyEscaped(self.value)
        } else {
            PreparedText::NeedsEscaped(self.value)
        }
    }
}

impl<'a, T> TextHandler<'a, MarkdownEscaper, T> for TextHandlerForMarkdown<'a, T> {
    #[inline]
    fn prepare_text_for_output(&self, _escaper: &MarkdownEscaper) -> PreparedText<'a, T> {
        PreparedText::NeedsEscaped(self.value)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use oxiplate_traits::PreparedText;

    use super::{IsEscapedForMarkdown, MarkdownEscaper, TextHandlerForMarkdown};
    use crate::TextHandler as _;

    #[derive(PartialEq, Eq)]
    struct NotEscapedForMarkdown;

    impl IsEscapedForMarkdown for NotEscapedForMarkdown {
        fn is_escaped(_escaper: &MarkdownEscaper) -> bool {
            false
        }
    }

    #[test]
    fn not_escaped_for_markdown() {
        let PreparedText::NeedsEscaped(&NotEscapedForMarkdown) = (&&TextHandlerForMarkdown {
            value: &NotEscapedForMarkdown,
        })
            .prepare_text_for_output(&MarkdownEscaper::text)
        else {
            panic!("Expected value to need escaping");
        };
    }
}
