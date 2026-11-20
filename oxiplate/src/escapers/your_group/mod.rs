//! Example escaper used in `crate::escapers` documentation.

mod escapers;
mod is_escaped;
mod text_handler;

use core::fmt::{Result, Write};

use self::escapers::{bar_escaper, escape_foo};
use self::text_handler::TextHandlerForYourGroup;
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

    type TextHandler<'a, T: 'a> = TextHandlerForYourGroup<'a, T>;

    #[inline]
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
        match self {
            Self::foo => escape_foo(f, value),
            Self::bar => bar_escaper(f, value),
        }
    }

    fn build_text_handler<T>(value: &T) -> Self::TextHandler<'_, T> {
        TextHandlerForYourGroup { value }
    }
}
