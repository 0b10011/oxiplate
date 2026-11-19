extern crate alloc;

use super::YourEscaper;

pub(super) trait IsEscapedForYourGroup {
    fn is_escaped(escaper: &YourEscaper) -> bool;
}

impl<T> IsEscapedForYourGroup for &T
where
    T: IsEscapedForYourGroup,
{
    fn is_escaped(escaper: &YourEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl<T> IsEscapedForYourGroup for &mut T
where
    T: IsEscapedForYourGroup,
{
    fn is_escaped(escaper: &YourEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl<T> IsEscapedForYourGroup for alloc::boxed::Box<T>
where
    T: IsEscapedForYourGroup,
{
    fn is_escaped(escaper: &YourEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl<T> IsEscapedForYourGroup for alloc::rc::Rc<T>
where
    T: IsEscapedForYourGroup,
{
    fn is_escaped(escaper: &YourEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl<T> IsEscapedForYourGroup for alloc::sync::Arc<T>
where
    T: IsEscapedForYourGroup,
{
    fn is_escaped(escaper: &YourEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl IsEscapedForYourGroup for bool {
    fn is_escaped(escaper: &YourEscaper) -> bool {
        match escaper {
            YourEscaper::foo | YourEscaper::bar => true,
        }
    }
}

macro_rules! numbers_safe_for_your_group {
    ($($ty:ty)*) => { $(
        impl IsEscapedForYourGroup for $ty {
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
