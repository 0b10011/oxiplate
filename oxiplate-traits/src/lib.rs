#![no_std]
#![doc(issue_tracker_base_url = "https://github.com/0b10011/oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod cow_str;
mod escaper;
mod render;
mod unescaped_text;

pub use self::cow_str::{CowStr, FastCowStr};
pub use self::escaper::{Escaper, PreparedText, TextHandler};
pub use self::render::Render;
pub use self::unescaped_text::{FastWriteEscaped, FastWriteRaw};

/// Items only meant to be used by `oxiplate-derive` in code generation.
pub mod private {
    pub use super::cow_str::{CowStrWrapper, ToCowStr, ToCowStrWrapper};
    pub use super::unescaped_text::{UnescapedTextWrapper, WriteEscaped, WriteRaw};
}

/// Macro to efficiently convert a value to a `CowStrWrapper`
/// for unit testing filters that deal with string-like values.
///
/// Oxiplate does this conversion for you when the cow prefix (`>`) is present,
/// and filters only need to import the `CowStr` trait for production code.
///
/// # Example
///
/// ```
/// extern crate alloc;
///
/// use alloc::borrow::Cow;
///
/// use oxiplate_traits::private::ToCowStr as _;
/// use oxiplate_traits::{CowStr, cow_str_wrapper};
///
/// pub fn upper<'a, E: CowStr<'a>>(expression: E) -> String {
///     match expression.cow_str() {
///         Cow::Borrowed(str) => str.to_uppercase(),
///         Cow::Owned(string) => string.to_uppercase(),
///     }
/// }
///
/// #[test]
/// # fn test_doesnt_run() {}
/// fn str() {
///     assert_eq!("HELLO WORLD!", upper(cow_str_wrapper!("Hello World!")));
/// }
/// # str();
///
/// #[test]
/// # fn test_doesnt_run_2() {}
/// fn string() {
///     assert_eq!(
///         "HELLO WORLD!",
///         upper(cow_str_wrapper!(String::from("Hello World!")))
///     );
/// }
/// # string();
/// ```
#[macro_export]
#[allow(clippy::test_attr_in_doctest)]
macro_rules! cow_str_wrapper {
    ($expr:expr) => {
        ::oxiplate_traits::private::CowStrWrapper::new(
            (&&::oxiplate_traits::private::ToCowStrWrapper::new(&($expr))).to_cow_str(),
        )
    };
}
