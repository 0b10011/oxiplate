//! Built-in filters.
//!
//! # Build your own filter
//!
//! Filters are normal Rust functions
//! where the first argument is the expression before the filter
//! and the second one is the first passed to the filter in the template.
//!
//! ```rust
//! use oxiplate::{Oxiplate, Render};
//!
//! mod filters_for_oxiplate {
//!     use std::borrow::Cow;
//!
//!     use oxiplate::CowStr;
//!
//!     /// Your custom filter.
//!     /// `expression` is the expression preceding the filter in the template.
//!     /// `replacement` is the first argument passed in the template.
//!     ///
//!     /// In the following example,
//!     /// `>some_variable` is passed in for `expression`
//!     /// and `"·"` is passed in for `replacement`:
//!     ///
//!     /// ```oxip
//!     /// {{ >some_variable | your_filter("·") }}`
//!     /// ```
//!     ///
//!     /// The `CowStr` trait is used for more efficient string conversion.
//!     pub fn your_filter<'a, E: CowStr<'a>>(expression: E, replacement: &str) -> Cow<'a, str> {
//!         let expression = expression.cow_str();
//!
//!         if expression.contains(' ') {
//!             expression.replace(' ', replacement).into()
//!         } else {
//!             expression
//!         }
//!     }
//! }
//!
//! fn main() -> Result<(), ::core::fmt::Error> {
//!     #[derive(Oxiplate)]
//!     #[oxiplate_inline(html: r#"{{ >"hello world!" | your_filter("·") }}"#)]
//!     struct Data;
//!
//!     assert_eq!(Data.render()?, "hello·world!");
//!
//!     Ok::<(), ::core::fmt::Error>(())
//! }
//! ```

mod default;
mod join;
mod r#loop;
mod lower;
mod trim;
mod upper;

pub use default::default;
pub use join::join;
pub use r#loop::r#loop;
pub use lower::lower;
pub use trim::{trim, trim_end, trim_start};
pub use upper::upper;

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
/// #[cfg(test)]
/// # compile_error!("tests don't run in docs");
/// use oxiplate::{cow_str_wrapper, private::ToCowStr};
/// use oxiplate_traits::CowStr;
///
/// pub fn upper<'a, E: CowStr<'a>>(expression: E) -> Cow<'a, str> {
///     match expression.cow_str() {
///         Cow::Borrowed(str) => str.to_uppercase().into(),
///         Cow::Owned(string) => string.to_uppercase().into(),
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
#[cfg(test)]
macro_rules! cow_str_wrapper {
    ($expr:expr) => {
        ::oxiplate_traits::private::CowStrWrapper::new(
            (&&::oxiplate_traits::private::ToCowStrWrapper::new(&($expr))).to_cow_str(),
        )
    };
}

#[cfg(test)]
pub(self) use cow_str_wrapper;
