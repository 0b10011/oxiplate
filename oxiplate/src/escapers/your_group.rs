//! Example escaper used in `crate::escapers` documentation.

use std::fmt::{Result, Write};

use super::Escaper;

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

    #[inline]
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
        match self {
            Self::foo => escape_foo(f, value),
            Self::bar => bar_escaper(f, value),
        }
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
