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

/// Macro to quickly build the boilerplate for `IsEscapedForEscaper` for an escaper.
///
/// # Usage
///
/// ```rust
/// # use oxiplate_traits as oxiplate;
/// # #[expect(dead_code, non_camel_case_types)]
/// # enum HtmlEscaper { text, attr, comment }
/// extern crate alloc;
///
/// use oxiplate::is_escaped;
///
/// is_escaped!(pub IsEscapedForHtml, HtmlEscaper);
///
/// // Add any additional implementations for your own or external types:
/// impl IsEscapedForHtml for bool {
///     fn is_escaped(escaper: &HtmlEscaper) -> bool {
///         // While booleans may be safe for all contexts,
///         // this won't always be the case.
///         // Adjust this match as needed for each type.
///         match escaper {
///             HtmlEscaper::text | HtmlEscaper::attr | HtmlEscaper::comment => true,
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! is_escaped {
    ($visibility:vis $trait_name:ident, $escaper:path) => {
        /// Trait to check if a value is already escaped.
        $visibility trait $trait_name {
            /// Check if the value is already escaped.
            #[must_use]
            fn is_escaped(escaper: &$escaper) -> bool;
        }

        impl<T> $trait_name for &T
        where
            T: $trait_name,
        {
            #[inline]
            fn is_escaped(escaper: &$escaper) -> bool {
                <T>::is_escaped(escaper)
            }
        }

        impl<T> $trait_name for &mut T
        where
            T: $trait_name,
        {
            #[inline]
            fn is_escaped(escaper: &$escaper) -> bool {
                <T>::is_escaped(escaper)
            }
        }

        impl<T> $trait_name for alloc::boxed::Box<T>
        where
            T: $trait_name,
        {
            #[inline]
            fn is_escaped(escaper: &$escaper) -> bool {
                <T>::is_escaped(escaper)
            }
        }

        impl<T> $trait_name for alloc::rc::Rc<T>
        where
            T: $trait_name,
        {
            #[inline]
            fn is_escaped(escaper: &$escaper) -> bool {
                <T>::is_escaped(escaper)
            }
        }

        impl<T> $trait_name for alloc::sync::Arc<T>
        where
            T: $trait_name,
        {
            #[inline]
            fn is_escaped(escaper: &$escaper) -> bool {
                <T>::is_escaped(escaper)
            }
        }
    };
}
