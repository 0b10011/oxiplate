//! Built-in escapers.
//!
//! # Build your own escaper
//!
//! To build your own escaper,
//! make an enum with variants matching the escaper names you'd like to use,
//! and have it implement [`crate::escapers::Escaper`].
//! Oxiplate templates usually use `snake_case` escaper names,
//! so it's suggested to use those on your escaper group enum.
//!
//! ```rust
//! use std::fmt::{Result, Write};
//!
//! use oxiplate::escapers::Escaper;
//!
//! # #[allow(dead_code)]
//! #[allow(non_camel_case_types)]
//! pub enum YourEscaper {
//! #   #[allow(dead_code)]
//!     foo,
//! #   #[allow(dead_code)]
//!     bar,
//! }
//!
//! impl Escaper for YourEscaper {
//!     const DEFAULT: Self = Self::foo;
//!
//!     #[inline]
//!     fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
//!         match self {
//!             Self::foo => escape_foo(f, value),
//!             Self::bar => bar_escaper(f, value),
//!         }
//!     }
//! }
//!
//! # #[allow(dead_code)]
//! #[inline]
//! fn escape_foo<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
//!     if !value.contains("foo") {
//!         return f.write_str(value);
//!     }
//!
//!     f.write_str(&value.replace("foo", "f00"))
//! }
//!
//! # #[allow(dead_code)]
//! #[inline]
//! fn bar_escaper<W: Write + ?Sized>(f: &mut W, value: &'_ str) -> Result {
//!     if !value.contains("bar") {
//!         return f.write_str(value);
//!     }
//!
//!     f.write_str(&value.replace("bar", "b@r"))
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
//! ```
//! use oxiplate::{Oxiplate, Render};
//!
//! #[derive(Oxiplate)]
//! // Because the TOML isn't included in this,
//! // `your_group` isn't available and the example doesn't compile.
//! #[oxiplate_inline(your_group: "{{ foo: value_to_escape }} | {{ bar: value_to_escape }}")]
//! struct Data<'a> {
//!     value_to_escape: &'a str,
//! }
//!
//! let data = Data {
//!     value_to_escape: "foo bar",
//! };
//!
//! assert_eq!(data.render()?, r#"f00 bar | foo b@r"#);
//!
//! # Ok::<(), ::core::fmt::Error>(())
//! ```

pub mod html;
pub mod json;
pub mod markdown;
pub mod your_group;

use std::fmt::{Result, Write};

/// Trait for an Oxiplate-compatible escaper group.
pub trait Escaper {
    /// The default escaper for this escaper group.
    const DEFAULT: Self;

    /// Function that escapes text based on the selected variant.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result;
}
