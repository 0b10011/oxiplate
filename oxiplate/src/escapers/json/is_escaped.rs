extern crate alloc;

use super::JsonEscaper;

/// Trait to check if a value is already escaped.
pub trait IsEscapedForJson {
    /// Check if the value is already escaped.
    fn is_escaped(escaper: &JsonEscaper) -> bool;
}

impl<T> IsEscapedForJson for &T
where
    T: IsEscapedForJson,
{
    fn is_escaped(escaper: &JsonEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl<T> IsEscapedForJson for &mut T
where
    T: IsEscapedForJson,
{
    fn is_escaped(escaper: &JsonEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl<T> IsEscapedForJson for alloc::boxed::Box<T>
where
    T: IsEscapedForJson,
{
    fn is_escaped(escaper: &JsonEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl<T> IsEscapedForJson for alloc::rc::Rc<T>
where
    T: IsEscapedForJson,
{
    fn is_escaped(escaper: &JsonEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

impl<T> IsEscapedForJson for alloc::sync::Arc<T>
where
    T: IsEscapedForJson,
{
    fn is_escaped(escaper: &JsonEscaper) -> bool {
        <T>::is_escaped(escaper)
    }
}

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
