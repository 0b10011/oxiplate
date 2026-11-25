use super::{XmlEscaper, IsEscapedForXml};
use crate::{PreparedText, TextHandler};

/// Item that implements `TextHandler` for `XmlEscaper`.
pub struct TextHandlerForXml<'a, T> {
    pub(super) value: &'a T,
}

impl<'a, T> TextHandler<'a, XmlEscaper, T> for &TextHandlerForXml<'a, T>
where
    T: IsEscapedForXml,
{
    #[inline]
    fn prepare_text_for_output(&self, escaper: &XmlEscaper) -> PreparedText<'a, T> {
        if <T>::is_escaped(escaper) {
            PreparedText::AlreadyEscaped(self.value)
        } else {
            PreparedText::NeedsEscaped(self.value)
        }
    }
}

impl<'a, T> TextHandler<'a, XmlEscaper, T> for TextHandlerForXml<'a, T> {
    #[inline]
    fn prepare_text_for_output(&self, _escaper: &XmlEscaper) -> PreparedText<'a, T> {
        PreparedText::NeedsEscaped(self.value)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use oxiplate_traits::PreparedText;

    use super::{XmlEscaper, IsEscapedForXml, TextHandlerForXml};
    use crate::TextHandler as _;

    #[derive(PartialEq, Eq)]
    struct NotEscapedForXml;

    impl IsEscapedForXml for NotEscapedForXml {
        fn is_escaped(_escaper: &XmlEscaper) -> bool {
            false
        }
    }

    #[test]
    fn not_escaped_for_xml() {
        let PreparedText::NeedsEscaped(&NotEscapedForXml) = (&&TextHandlerForXml {
            value: &NotEscapedForXml,
        })
            .prepare_text_for_output(&XmlEscaper::text)
        else {
            panic!("Expected value to need escaping");
        };
    }
}
