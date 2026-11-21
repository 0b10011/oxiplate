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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use oxiplate_traits::PreparedText;

    use super::{HtmlEscaper, IsEscapedForHtml, TextHandlerForHtml};
    use crate::TextHandler as _;

    #[derive(PartialEq, Eq)]
    struct NotEscapedForHtml;

    impl IsEscapedForHtml for NotEscapedForHtml {
        fn is_escaped(_escaper: &HtmlEscaper) -> bool {
            false
        }
    }

    #[test]
    fn not_escaped_for_html() {
        let PreparedText::NeedsEscaped(&NotEscapedForHtml) = (&&TextHandlerForHtml {
            value: &NotEscapedForHtml,
        })
            .prepare_text_for_output(&HtmlEscaper::text)
        else {
            panic!("Expected value to need escaping");
        };
    }
}
