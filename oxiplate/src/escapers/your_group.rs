//! Example escaper used in `crate::escapers` documentation.

use core::fmt::{Result, Write};

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

    fn build_is_escaped<T>(value: &T) -> Self::IsEscaped<'_, T> {
        IsEscapedForYourEscaper { value }
    }
}

/// Item that implements `IsEscaped` for `YourEscaper`.
pub struct IsEscapedForYourEscaper<'a, T> {
    value: &'a T,
}

impl<'a> IsEscaped<'a, YourEscaper, usize> for &IsEscapedForYourEscaper<'a, usize> {
    fn is_escaped(&self, escaper: &YourEscaper) -> (&'a usize, bool) {
        match escaper {
            YourEscaper::foo | YourEscaper::bar => (self.value, true),
        }
    }
}

impl<'a, T> IsEscaped<'a, YourEscaper, T> for IsEscapedForYourEscaper<'a, T> {
    fn is_escaped(&self, _escaper: &YourEscaper) -> (&'a T, bool) {
        (self.value, false)
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
