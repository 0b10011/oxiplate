extern crate alloc;

use super::HtmlEscaper;

/// Trait to check if a value is already escaped.
pub trait IsEscapedForHtml {
    /// Check if the value is already escaped.
    fn is_escaped(escaper: &HtmlEscaper) -> bool;
}

impl<T> IsEscapedForHtml for &T
where
    T: IsEscapedForHtml,
{
    fn is_escaped(escaper: &HtmlEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl<T> IsEscapedForHtml for &mut T
where
    T: IsEscapedForHtml,
{
    fn is_escaped(escaper: &HtmlEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl<T> IsEscapedForHtml for alloc::boxed::Box<T>
where
    T: IsEscapedForHtml,
{
    fn is_escaped(escaper: &HtmlEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl<T> IsEscapedForHtml for alloc::rc::Rc<T>
where
    T: IsEscapedForHtml,
{
    fn is_escaped(escaper: &HtmlEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl<T> IsEscapedForHtml for alloc::sync::Arc<T>
where
    T: IsEscapedForHtml,
{
    fn is_escaped(escaper: &HtmlEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

macro_rules! numbers_safe_for_html {
    ($($ty:ty)*) => { $(
        impl IsEscapedForHtml for $ty {
            fn is_escaped(escaper: &HtmlEscaper) -> bool {
                match escaper {
                    HtmlEscaper::text | HtmlEscaper::attr | HtmlEscaper::comment => true,
                }
            }
        }
    )* };
}

numbers_safe_for_html!(
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
    f32 f64
);
