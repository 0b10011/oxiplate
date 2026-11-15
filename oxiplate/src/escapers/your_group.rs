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

    type IsEscaped<T> = IsEscapedForYourEscaper<T>;

    #[inline]
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
        match self {
            Self::foo => escape_foo(f, value),
            Self::bar => bar_escaper(f, value),
        }
    }

    fn build_is_escaped<T>(_value: &T) -> Self::IsEscaped<T> {
        IsEscapedForYourEscaper {
            phantom_data: PhantomData,
        }
    }
}

/// Item that implements `IsEscaped` for `YourEscaper`.
pub struct IsEscapedForYourEscaper<T> {
    phantom_data: PhantomData<T>,
}

impl IsEscaped<YourEscaper> for &IsEscapedForYourEscaper<usize> {
    fn is_escaped(&self, escaper: &YourEscaper) -> bool {
        match escaper {
            YourEscaper::foo | YourEscaper::bar => true,
        }
    }
}

impl<T> IsEscaped<YourEscaper> for IsEscapedForYourEscaper<T> {
    fn is_escaped(&self, _escaper: &YourEscaper) -> bool {
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
