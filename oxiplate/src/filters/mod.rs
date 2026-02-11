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
mod r#loop;
mod lower;
mod trim;
mod upper;

pub use default::default;
pub use r#loop::r#loop;
pub use lower::lower;
pub use trim::{trim, trim_end, trim_start};
pub use upper::upper;
