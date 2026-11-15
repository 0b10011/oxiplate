//! Example escaper used in `crate::escapers` documentation.

use core::fmt::{Result, Write};
use core::marker::PhantomData;

use oxiplate_traits::{Escaper, IsEscaped, IsSafeForBuilder};

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

    type Builder<'a, T: 'a> = IsSafeForYourEscaperBuilder<'a, T>;

    #[inline]
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
        match self {
            Self::foo => escape_foo(f, value),
            Self::bar => bar_escaper(f, value),
        }
    }

    fn is_safe_for_builder<'a, T>() -> Self::Builder<'a, T> {
        IsSafeForYourEscaperBuilder {
            phantom_data: PhantomData,
        }
    }
}

/// Builder for `IsSafeForYourEscaper`.
pub struct IsSafeForYourEscaperBuilder<'a, T> {
    phantom_data: PhantomData<&'a T>,
}

impl<'a, T> IsSafeForBuilder<'a, T> for IsSafeForYourEscaperBuilder<'a, T> {
    type IsEscaped = IsSafeForYourEscaper<'a, T>;

    fn build(&'a self, value: &'a T) -> Self::IsEscaped {
        IsSafeForYourEscaper::new(value)
    }
}

/// Item that implements `IsEscaped` for `YourEscaper`.
pub struct IsSafeForYourEscaper<'a, T: ?Sized> {
    phantom_data: PhantomData<&'a T>,
}

impl<'a, T> IsSafeForYourEscaper<'a, T> {
    /// Build `IsSafeForYourEscaper` for the provided value.
    pub fn new(_value: &'a T) -> Self {
        Self {
            phantom_data: PhantomData,
        }
    }
}

impl IsEscaped for &IsSafeForYourEscaper<'_, usize> {
    type Escaper = YourEscaper;

    fn is_escaped(&self, escaper: &Self::Escaper) -> bool {
        match escaper {
            YourEscaper::foo | YourEscaper::bar => true,
        }
    }
}

impl<T> IsEscaped for IsSafeForYourEscaper<'_, T> {
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
