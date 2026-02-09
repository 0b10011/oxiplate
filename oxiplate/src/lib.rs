#![no_std]
#![doc(issue_tracker_base_url = "https://github.com/0b10011/oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod escapers;
pub mod filters;

pub use oxiplate_derive::Oxiplate;
pub use oxiplate_traits::{
    CowStr, CowStrWrapper, Escaper, FastCowStr, FastEscape, Render, ToCowStr, ToCowStrWrapper,
    UnescapedText, UnescapedTextWrapper,
};

/// Default Oxiplate experience that uses only built-in filters.
///
/// To use your own filters:
///
/// ```
/// // Create a module named `filters_for_oxiplate` at the root of _your_ crate
/// // to hold the filters.
/// mod filters_for_oxiplate {
///     // Include all of the default filters:
///     // Or just those you want to use:
/// #   #[allow(unused_imports)]
///     pub use oxiplate::filters::{upper, *};
///
///     // And then add/import your filters
///     pub fn lower(value: &str) -> String {
///         value.to_lowercase()
///     }
/// }
///
/// // And import those filters where they're needed:
/// mod your_mod {
///     use std::fmt::Error;
///
///     use oxiplate::{Oxiplate, Render};
///
///     #[derive(Oxiplate)]
///     #[oxiplate_inline(html: r#"{{ "Hello World" | lower() }}"#)]
///     struct Data;
///
///     pub fn data() -> Result<(), Error> {
///         assert_eq!(Data.render()?, "hello world");
///         Ok(())
///     }
/// }
/// # fn main() -> Result<(), std::fmt::Error> {
/// #     your_mod::data()
/// # }
/// ```
pub mod prelude {
    pub use super::{filters as filters_for_oxiplate, Oxiplate, Render};
}
