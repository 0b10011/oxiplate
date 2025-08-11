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
//! use std::fmt::{Result, Write};
//!
//! use oxiplate::escapers::Escaper;
//!
//! # #[allow(dead_code)]
//! pub enum YourEscaper {
//! #   #[allow(dead_code)]
//!     Foo,
//! #   #[allow(dead_code)]
//!     Bar,
//! }
//!
//! impl Escaper for YourEscaper {
//!     const DEFAULT: Self = Self::Foo;
//!
//!     #[inline]
//!     fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result {
//!         match self {
//!             Self::Foo => escape_foo(f, value),
//!             Self::Bar => bar_escaper(f, value),
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
//! ```compile_fail
//! use oxiplate::{Oxiplate, Render};
//!
//! #[derive(Oxiplate)]
//! // Because the TOML isn't included in this,
//! // `your_group` isn't available and the example doesn't compile.
//! #[oxiplate_inline(html: "{{ your_group.foo: value_to_escape }}")]
//! struct Data<'a> {
//!     value_to_escape: &'a str,
//! }
//!
//! let data = Data {
//!     value_to_escape: "<.<",
//! };
//!
//! assert_eq!(data.render()?, r#"&lt;.&lt;"#);
//!
//! Ok::<(), ::core::fmt::Error>(())
//! ```

pub mod html;
// pub mod json;
// pub mod markdown;

use std::{fmt::Display, io::{Result, Write}};

/// Trait for an Oxiplate-compatible escaper group.
pub trait Escaper {
    /// The default escaper for this escaper group.
    const DEFAULT: Self;

    /// Function that escapes text based on the selected variant.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &[u8]) -> Result<()>;
}

/// Helper function to ensure the provided escaper implements [`Escaper`].
/// Called from generated templates whenever an escaper is used.
///
/// # Errors
///
/// If escaped string cannot be written to the writer.
#[inline]
pub fn escape<W: Write + ?Sized>(f: &mut W, escaper: &impl Escaper, text: &[u8]) -> Result<()> {
    escaper.escape(f, text)
}

/// Specialized calls to escape.
pub trait UnescapedText<'a, W: Write + ?Sized> {
    /// Helper function to ensure the provided escaper implements [`Escaper`]
    /// and to use the most efficient conversion to `&str`.
    /// Called from generated templates whenever an escaper is used.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result<()>;

    /// Helper function to use the most efficient conversion to `&str`.
    /// Called from generated templates whenever raw output is used.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn raw(&'a self, f: &mut W) -> Result<()>;
}

impl<'a, T: ToString + Display, W: Write + ?Sized> UnescapedText<'a, W> for &T {
    #[inline]
    fn escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result<()> {
        escaper.escape(f, ::std::string::ToString::to_string(&self).as_bytes())
    }

    #[inline]
    fn raw(&'a self, f: &mut W) -> Result<()> {
        f.write_all(::std::string::ToString::to_string(&self).as_bytes())
    }
}

impl<'a, W: Write + ?Sized> UnescapedText<'a, W> for String {
    #[inline]
    fn escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result<()> {
        escaper.escape(f, self.as_bytes())
    }

    #[inline]
    fn raw(&'a self, f: &mut W) -> Result<()> {
        f.write_all(self.as_bytes())
    }
}

impl<'a, W: Write + ?Sized> UnescapedText<'a, W> for &str {
    #[inline]
    fn escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result<()> {
        escaper.escape(f, self.as_bytes())
    }

    #[inline]
    fn raw(&'a self, f: &mut W) -> Result<()> {
        f.write_all(self.as_bytes())
    }
}

impl<'a, W: Write + ?Sized> UnescapedText<'a, W> for &[u8] {
    #[inline]
    fn escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result<()> {
        escaper.escape(f, self)
    }

    #[inline]
    fn raw(&'a self, f: &mut W) -> Result<()> {
        f.write_all(self)
    }
}
