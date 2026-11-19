extern crate alloc;

use super::MarkdownEscaper;

/// Trait to check if a value is already escaped.
pub trait IsEscapedForMarkdown {
    /// Check if the value is already escaped.
    fn is_escaped(escaper: &MarkdownEscaper) -> bool;
}
impl<T> IsEscapedForMarkdown for &T
where
    T: IsEscapedForMarkdown,
{
    fn is_escaped(escaper: &MarkdownEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}
impl<T> IsEscapedForMarkdown for &mut T
where
    T: IsEscapedForMarkdown,
{
    fn is_escaped(escaper: &MarkdownEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}
impl<T> IsEscapedForMarkdown for alloc::boxed::Box<T>
where
    T: IsEscapedForMarkdown,
{
    fn is_escaped(escaper: &MarkdownEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}
impl<T> IsEscapedForMarkdown for alloc::rc::Rc<T>
where
    T: IsEscapedForMarkdown,
{
    fn is_escaped(escaper: &MarkdownEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}
impl<T> IsEscapedForMarkdown for alloc::sync::Arc<T>
where
    T: IsEscapedForMarkdown,
{
    fn is_escaped(escaper: &MarkdownEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl IsEscapedForMarkdown for bool {
    fn is_escaped(escaper: &MarkdownEscaper) -> bool {
        match escaper {
            MarkdownEscaper::text => true,
        }
    }
}
