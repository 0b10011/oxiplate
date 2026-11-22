extern crate alloc;

use super::HtmlEscaper;
use crate::is_escaped;

is_escaped!(pub IsEscapedForHtml, HtmlEscaper);

impl IsEscapedForHtml for bool {
    #[inline]
    fn is_escaped(escaper: &HtmlEscaper) -> bool {
        match escaper {
            HtmlEscaper::text | HtmlEscaper::attr | HtmlEscaper::comment => true,
        }
    }
}

macro_rules! numbers_safe_for_html {
    ($($ty:ty)*) => { $(
        impl IsEscapedForHtml for $ty {
            #[inline]
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
