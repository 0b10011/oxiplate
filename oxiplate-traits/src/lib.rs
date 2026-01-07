#![doc(issue_tracker_base_url = "https://github.com/0b10011/oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod cow_str;
mod escaper;
mod render;
mod unescaped_text;

pub use cow_str::{CowStr, CowStrWrapper, FastCowStr, ToCowStr, ToCowStrWrapper};
pub use escaper::Escaper;
pub use render::Render;
pub use unescaped_text::{FastEscape, UnescapedText, UnescapedTextWrapper};

/// Macro to efficiently convert a value to a `CowStrWrapper`
/// for unit testing filters that deal with string-like values.
///
/// Oxiplate does this conversion for you when the cow prefix (`>`) is present,
/// and filters only need to import the `CowStr` trait for production code.
///
/// # Example
///
/// ```
/// use std::borrow::Cow;
///
/// use oxiplate_traits::CowStr;
/// #[cfg(test)]
/// # compile_error!("tests don't run in docs");
/// use oxiplate_traits::{ToCowStr, cow_str_wrapper};
///
/// pub fn upper<'a, E: CowStr<'a>>(expression: E) -> Cow<'a, str> {
///     match expression.cow_str() {
///         Cow::Borrowed(str) => str.to_uppercase().into(),
///         Cow::Owned(string) => string.to_uppercase().into(),
///     }
/// }
///
/// # #[allow(clippy::test_attr_in_doctest)]
/// #[test]
/// # fn test_doesnt_run() {}
/// fn str() {
///     assert_eq!("HELLO WORLD!", upper(cow_str_wrapper!("Hello World!")));
/// }
/// # str();
///
/// # #[allow(clippy::test_attr_in_doctest)]
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
macro_rules! cow_str_wrapper {
    ($expr:expr) => {
        ::oxiplate_traits::CowStrWrapper::new(
            (&&::oxiplate_traits::ToCowStrWrapper::new(&($expr))).to_cow_str(),
        )
    };
}
