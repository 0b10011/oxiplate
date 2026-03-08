#![no_std]
#![doc(issue_tracker_base_url = "https://github.com/0b10011/oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

#[cfg(feature = "translation")]
extern crate alloc;

pub mod escapers;
pub mod filters;

#[cfg(feature = "translation")]
pub use linkme::distributed_slice;
pub use oxiplate_derive::Oxiplate;
pub use oxiplate_traits::{
    CowStr, CowStrWrapper, Escaper, FastCowStr, FastEscape, Render, ToCowStr, ToCowStrWrapper,
    UnescapedText, UnescapedTextWrapper,
};

/// Text gathered from a template that should be translated.
#[cfg(feature = "translation")]
pub type Translation = (&'static str, &'static str);

/// Collection of text gathered from a template
/// that should be translated gathered from a template.
#[cfg(feature = "translation")]
pub type Translations = alloc::vec::Vec<Translation>;

/// Collection of text gathered from a template
/// that should be translated gathered from a template.
#[cfg(feature = "translation")]
pub type TranslationsSignature = fn() -> Translations;

/// Trait templates implement
/// to provide the list of translatable text
/// within the template.
#[cfg(feature = "translation")]
pub trait TranslationExtractor {
    /// Get the list of translatable text and associated context, if any.
    fn translations() -> Translations;
}

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
    pub use super::{Oxiplate, Render, filters as filters_for_oxiplate};
    #[cfg(feature = "translation")]
    pub use super::{Translation, Translations, TranslationsSignature, distributed_slice};
}
