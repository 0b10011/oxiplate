extern crate alloc;

use super::YourEscaper;
use crate::is_escaped;

is_escaped!(pub(super) IsEscapedForYourGroup, YourEscaper);

impl IsEscapedForYourGroup for bool {
    #[inline]
    fn is_escaped(escaper: &YourEscaper) -> bool {
        match escaper {
            YourEscaper::foo => true,
            YourEscaper::bar => false,
        }
    }
}

macro_rules! numbers_safe_for_your_group {
    ($($ty:ty)*) => { $(
        impl IsEscapedForYourGroup for $ty {
            #[inline]
            fn is_escaped(escaper: &YourEscaper) -> bool {
                match escaper {
                    YourEscaper::foo | YourEscaper::bar => true,
                }
            }
        }
    )* };
}

numbers_safe_for_your_group!(
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
    f32 f64
);
