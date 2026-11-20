//! Example escaper used in `crate::escapers` documentation.

mod escapers;

use core::fmt::{Result, Write};

use self::escapers::{bar_escaper, escape_foo};
use crate::Escaper;

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
