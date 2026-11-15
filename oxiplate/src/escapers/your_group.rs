//! Example escaper used in `crate::escapers` documentation.

use core::fmt::{Result, Write};
use core::marker::PhantomData;

use oxiplate_traits::{Escaper, IsEscaped};

/// Enum for example escaper.
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum YourEscaper {
    /// `foo` escaper that replaces "foo" with "f00".
    foo,
    /// `bar` escaper that replaces "bar" with "b@r".
    bar,
}

impl Escaper for YourEscaper {
    const DEFAULT: Self = Self::foo;

    type IsEscaped<'a, T: 'a> = IsEscapedForYourEscaper<'a, T>;

    #[inline]
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
        match self {
            Self::foo => escape_foo(f, value),
            Self::bar => bar_escaper(f, value),
        }
    }

    fn build_is_escaped<T>(_value: &T) -> Self::IsEscaped<'_, T> {
        IsEscapedForYourEscaper {
            phantom_data: PhantomData,
        }
    }
}

/// Item that implements `IsEscaped` for `YourEscaper`.
pub struct IsEscapedForYourEscaper<'a, T: ?Sized> {
    phantom_data: PhantomData<&'a T>,
}

impl IsEscaped for &IsEscapedForYourEscaper<'_, usize> {
    type Escaper = YourEscaper;

    fn is_escaped(&self, escaper: &Self::Escaper) -> bool {
        match escaper {
            YourEscaper::foo | YourEscaper::bar => true,
        }
    }
}

impl<T> IsEscaped for IsEscapedForYourEscaper<'_, T> {
    type Escaper = YourEscaper;

    fn is_escaped(&self, _escaper: &Self::Escaper) -> bool {
        false
    }
}

#[inline]
fn escape_foo<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
    if !value.contains("foo") {
        return f.write_str(value);
    }

    f.write_str(&value.replace("foo", "f00"))
}

#[inline]
fn bar_escaper<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
    if !value.contains("bar") {
        return f.write_str(value);
    }

    f.write_str(&value.replace("bar", "b@r"))
}
