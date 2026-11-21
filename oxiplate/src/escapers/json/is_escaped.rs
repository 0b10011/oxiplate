extern crate alloc;

use super::JsonEscaper;
use crate::is_escaped;

is_escaped!(pub IsEscapedForJson, JsonEscaper);

impl IsEscapedForJson for bool {
    fn is_escaped(escaper: &JsonEscaper) -> bool {
        match escaper {
            JsonEscaper::substring => true,
        }
    }
}

macro_rules! numbers_safe_for_json {
    ($($ty:ty)*) => { $(
        impl IsEscapedForJson for $ty {
            fn is_escaped(escaper: &JsonEscaper) -> bool {
                match escaper {
                    JsonEscaper::substring => true,
                }
            }
        }
    )* };
}

numbers_safe_for_json!(
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
    f32 f64
);
