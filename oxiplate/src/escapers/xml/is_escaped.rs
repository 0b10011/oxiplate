extern crate alloc;

use super::XmlEscaper;
use crate::is_escaped;

is_escaped!(pub IsEscapedForXml, XmlEscaper);

impl IsEscapedForXml for bool {
    #[inline]
    fn is_escaped(escaper: &XmlEscaper) -> bool {
        match escaper {
            XmlEscaper::text | XmlEscaper::attr | XmlEscaper::comment => true,
        }
    }
}

macro_rules! numbers_safe_for_xml {
    ($($ty:ty)*) => { $(
        impl IsEscapedForXml for $ty {
            #[inline]
            fn is_escaped(escaper: &XmlEscaper) -> bool {
                match escaper {
                    XmlEscaper::text | XmlEscaper::attr | XmlEscaper::comment => true,
                }
            }
        }
    )* };
}

numbers_safe_for_xml!(
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
    f32 f64
);
