use std::borrow::Cow;
use std::fmt;

use crate::{Escaper, FastEscape};

/// A trait intended to be used by filters for any string-like arguments
/// to deal with them more efficiently than `str` or `impl Display`.
/// Any expression or filter prefixed by `>` in a template
/// will convert the value to `CowStrWrapper`
/// which implements this trait.
///
/// # Example template
///
/// ```oxip
/// {{ >message | append(>"!") }}
/// ```
#[diagnostic::on_unimplemented(
    message = "{Self} is expected to be prefixed by `>` and implement \
               `::oxiplate_traits::ToCowString`.",
    label = "Insert `>` prefix before this expression if `{Self}` is a string-like value (`str`, \
             `impl Display`, etc).",
    note = "Consider implementing `::oxiplate_traits::ToCowStr` if `{Self}` is owned by your \
            crate, and then prefix with `>`.",
    note = "If `{Self}` is not a string-like value, perhaps you meant to pass a different \
            argument to the filter?",
    note = "Oxiplate efficiently builds `::std::borrow::Cow<'a, str>` when a `>` prefix is used \
            on an expression, and wraps it in `::oxiplate_traits::CowStr<'a>` which is used by \
            filters for string arguments. Because the syntax for doing so is not obvious and \
            prone to silently breaking, this alternative syntax is used to let Oxiplate handle \
            the details while ensuring it's tested properly."
)]
pub trait CowStr<'a> {
    /// Extract the contained `Cow<str>`.
    /// Intended to be called from filters
    /// to deal with strings more efficiently.
    #[must_use]
    fn cow_str(self) -> Cow<'a, str>;
}

/// Struct used in generated templates
/// that is intended to be passed to filters asking for `CowStr`
/// that will immediately extract the contained `Cow<str>`.
pub struct CowStrWrapper<'a>(Cow<'a, str>);

impl<'a> CowStrWrapper<'a> {
    /// Create a new `CowStrWrapper`.
    #[must_use]
    #[inline]
    pub fn new(cow_str: Cow<'a, str>) -> Self {
        Self(cow_str)
    }
}

impl<'a> CowStr<'a> for CowStrWrapper<'a> {
    #[inline]
    fn cow_str(self) -> Cow<'a, str> {
        self.0
    }
}

impl<'a, W: fmt::Write + ?Sized> FastEscape<'a, W> for CowStrWrapper<'a> {
    #[inline]
    fn oxiplate_fast_escape(&'a self, f: &mut W, escaper: &impl Escaper) -> fmt::Result {
        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str("CowStrWrapper(")?;

        escaper.escape(f, self.0.as_ref())?;

        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str(")")?;

        Ok(())
    }

    #[inline]
    fn oxiplate_fast_raw(&'a self, f: &mut W) -> fmt::Result {
        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str("CowStrWrapper(")?;

        f.write_str(self.0.as_ref())?;

        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

/// Wrapper around text
/// that will implement `CowStr`
/// via `FastCowStr` when possible,
/// otherwise via `::std::fmt::Display`.
/// Must borrow twice before calling `to_cow_str()`.
///
/// ```rust
/// # use oxiplate_traits as oxiplate;
/// use oxiplate::ToCowStr;
/// let text = "hello world";
/// assert_eq!(
///     "hello world",
///     (&&oxiplate::ToCowStrWrapper::new(&text)).to_cow_str()
/// );
/// ```
pub struct ToCowStrWrapper<'a, T>(&'a T);

impl<'a, T> ToCowStrWrapper<'a, T> {
    /// Wrap text.
    #[inline]
    pub fn new(value: &'a T) -> Self {
        Self(value)
    }
}

/// Trait with a specialized implementation
/// for items that implement `FastCowStr`,
/// a trait that allows for more efficient conversions to `Cow<'a, str>`.
pub trait ToCowStr<'a> {
    /// Helper function to use the most efficient conversion to `&str`.
    /// Called from generated templates whenever a cow prefix is used.
    fn to_cow_str(&'a self) -> Cow<'a, str>;
}

impl<'a, T: FastCowStr<'a>> ToCowStr<'a> for &ToCowStrWrapper<'a, T> {
    #[inline]
    fn to_cow_str(&'a self) -> Cow<'a, str> {
        self.0.oxiplate_cow_str()
    }
}

impl<'a, T: ToString> ToCowStr<'a> for &&ToCowStrWrapper<'a, T> {
    #[inline]
    fn to_cow_str(&'a self) -> Cow<'a, str> {
        Cow::Owned(self.0.to_string())
    }
}

/// Trait that allows for more efficient conversions to `&str`.
pub trait FastCowStr<'a> {
    /// Helper function to use the most efficient conversion to `&str`.
    /// Called from generated templates whenever a cow prefix is used.
    fn oxiplate_cow_str(&'a self) -> Cow<'a, str>;
}

impl<'a> FastCowStr<'a> for &'a str {
    #[inline]
    fn oxiplate_cow_str(&'a self) -> Cow<'a, str> {
        Cow::Borrowed(self)
    }
}
