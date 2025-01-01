//! Built-in escapers.
//!
//! # Build your own escaper
//!
//! To build your own escaper,
//! make an enum with variants matching the escaper names you'd like to use,
//! and have it implement [`crate::escapers::Escaper`].
//! While Oxiplate templates use `snake_case` escaper names,
//! they will be automatically converted to `PascalCase` when converted to an enum variant.
//!
//! ```rust
//! use oxiplate::escapers::Escaper;
//! use std::borrow::Cow;
//!
//! pub enum YourEscaper {
//! #   #[allow(dead_code)]
//!     Foo,
//! #   #[allow(dead_code)]
//!     Bar,
//! }
//!
//! impl Escaper for YourEscaper {
//!     fn escape<'a>(&self, value: &'a str) -> Cow<'a, str> {
//!         match self {
//!             Self::Foo => escape_foo(value),
//!             Self::Bar => bar_escaper(value),
//!         }
//!     }
//! }
//!
//! #[must_use]
//! fn escape_foo(value: &str) -> Cow<str> {
//!     if !value.contains("foo") {
//!         return Cow::Borrowed(value);
//!     }
//!
//!     value.replace("foo", "f00").into()
//! }
//!
//! #[must_use]
//! fn bar_escaper(value: &str) -> Cow<str> {
//!     if !value.contains("bar") {
//!         return Cow::Borrowed(value);
//!     }
//!
//!     value.replace("bar", "b@r").into()
//! }
//! ```
//!
//! ## TOML config
//!
//! ```toml
//! [escaper_groups.your_group]
//! escaper = "::your_crate::YourEscaper"
//! ```
//!
//! ## Rust code
//!
//! ```compile_fail
//! use oxiplate::Oxiplate;
//!
//! #[derive(Oxiplate)]
//! // Because the TOML isn't included in this,
//! // `your_group` isn't available and the example doesn't compile.
//! #[oxiplate_inline = "{{ your_group.foo: value_to_escape }}"]
//! struct Data<'a> {
//!     value_to_escape: &'a str,
//! }
//!
//! let data = Data {
//!     value_to_escape: "<.<",
//! };
//!
//! assert_eq!(format!("{}", data), r#"&lt;.&lt;"#);
//! ```

pub mod html;
pub mod markdown;

use std::borrow::Cow;

/// Trait for an Oxiplate-compatible escaper group.
pub trait Escaper {
    /// Function that escapes text based on the selected variant.
    fn escape<'a>(&self, value: &'a str) -> Cow<'a, str>;
}

/// Helper function to ensure the provided escaper implements [`Escaper`].
/// Called from generated templates whenever an escaper is used.
#[inline]
pub fn escape<'a>(escaper: &impl Escaper, text: &'a str) -> Cow<'a, str> {
    escaper.escape(text)
}