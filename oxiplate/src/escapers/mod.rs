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
pub mod json;
pub mod markdown;

use std::fmt::{Display, Result, Write};
use std::rc::Rc;
use std::sync::Arc;

/// Wrapper around escapable text.
pub struct UnescapedTextWrapper<'a, T: ?Sized>(&'a T);

impl<'a, T> UnescapedTextWrapper<'a, T> {
    /// Wrap escapable text.
    pub fn new(value: &'a T) -> Self {
        Self(value)
    }
}

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

/// Specialized calls to escape.
pub trait UnescapedText<'a, W: Write + ?Sized> {
    /// Helper function to ensure the provided escaper implements [`Escaper`]
    /// and to use the most efficient conversion to `&str`.
    /// Called from generated templates whenever an escaper is used.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn oxiplate_escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result;

    /// Helper function to use the most efficient conversion to `&str`.
    /// Called from generated templates whenever raw output is used.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn oxiplate_raw(&'a self, f: &mut W) -> Result;
}

/// Specialized calls to escape.
pub trait FastEscape<'a, W: Write + ?Sized> {
    /// Helper function to ensure the provided escaper implements [`Escaper`]
    /// and to use the most efficient conversion to `&str`.
    /// Called from generated templates whenever an escaper is used.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn oxiplate_fast_escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result;

    /// Helper function to use the most efficient conversion to `&str`.
    /// Called from generated templates whenever raw output is used.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn oxiplate_fast_raw(&'a self, f: &mut W) -> Result;
}

impl<'a, T: FastEscape<'a, W>, W: Write + ?Sized> UnescapedText<'a, W>
    for &UnescapedTextWrapper<'a, T>
{
    #[inline]
    fn oxiplate_escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result {
        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str("FastEscape(")?;

        self.0.oxiplate_fast_escape(f, escaper)?;

        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str(")")?;

        Ok(())
    }

    #[inline]
    fn oxiplate_raw(&'a self, f: &mut W) -> Result {
        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str("FastEscape(")?;

        self.0.oxiplate_fast_raw(f)?;

        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, T: ToString + Display, W: Write + ?Sized> UnescapedText<'a, W>
    for &&UnescapedTextWrapper<'a, T>
{
    #[inline]
    fn oxiplate_escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result {
        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str("Display(")?;

        escaper.escape(f, &::std::string::ToString::to_string(self.0))?;

        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str(")")?;

        Ok(())
    }

    #[inline]
    fn oxiplate_raw(&'a self, f: &mut W) -> Result {
        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str("Display(")?;

        f.write_str(&::std::string::ToString::to_string(self.0))?;

        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, T: FastEscape<'a, W> + ?Sized, W: Write + ?Sized> FastEscape<'a, W> for &T {
    #[inline]
    fn oxiplate_fast_escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result {
        <T>::oxiplate_fast_escape(self, f, escaper)
    }

    #[inline]
    fn oxiplate_fast_raw(&'a self, f: &mut W) -> Result {
        <T>::oxiplate_fast_raw(self, f)
    }
}

impl<'a, T: FastEscape<'a, W> + ?Sized, W: Write + ?Sized> FastEscape<'a, W> for &mut T {
    #[inline]
    fn oxiplate_fast_escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result {
        <T>::oxiplate_fast_escape(self, f, escaper)
    }

    #[inline]
    fn oxiplate_fast_raw(&'a self, f: &mut W) -> Result {
        <T>::oxiplate_fast_raw(self, f)
    }
}

impl<'a, T: FastEscape<'a, W> + ?Sized, W: Write + ?Sized> FastEscape<'a, W> for Box<T> {
    #[inline]
    fn oxiplate_fast_escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result {
        <T>::oxiplate_fast_escape(self.as_ref(), f, escaper)
    }

    #[inline]
    fn oxiplate_fast_raw(&'a self, f: &mut W) -> Result {
        <T>::oxiplate_fast_raw(self.as_ref(), f)
    }
}

impl<'a, T: FastEscape<'a, W> + ?Sized, W: Write + ?Sized> FastEscape<'a, W> for Rc<T> {
    #[inline]
    fn oxiplate_fast_escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result {
        <T>::oxiplate_fast_escape(self.as_ref(), f, escaper)
    }

    #[inline]
    fn oxiplate_fast_raw(&'a self, f: &mut W) -> Result {
        <T>::oxiplate_fast_raw(self.as_ref(), f)
    }
}

impl<'a, T: FastEscape<'a, W> + ?Sized, W: Write + ?Sized> FastEscape<'a, W> for Arc<T> {
    #[inline]
    fn oxiplate_fast_escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result {
        <T>::oxiplate_fast_escape(self.as_ref(), f, escaper)
    }

    #[inline]
    fn oxiplate_fast_raw(&'a self, f: &mut W) -> Result {
        <T>::oxiplate_fast_raw(self.as_ref(), f)
    }
}

impl<'a, W: Write + ?Sized> FastEscape<'a, W> for String {
    #[inline]
    fn oxiplate_fast_escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result {
        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str("String(")?;

        escaper.escape(f, self)?;

        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str(")")?;

        Ok(())
    }

    #[inline]
    fn oxiplate_fast_raw(&'a self, f: &mut W) -> Result {
        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str("String(")?;

        f.write_str(self)?;

        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, W: Write + ?Sized> FastEscape<'a, W> for str {
    #[inline]
    fn oxiplate_fast_escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result {
        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str("str(")?;

        escaper.escape(f, self)?;

        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str(")")?;

        Ok(())
    }

    #[inline]
    fn oxiplate_fast_raw(&'a self, f: &mut W) -> Result {
        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str("str(")?;

        f.write_str(self)?;

        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

macro_rules! unescaped_ints {
    ($($ty:ty)*) => { $(
        impl<'a, W: Write + ?Sized> FastEscape<'a, W> for $ty {
            #[inline]
            fn oxiplate_fast_escape(&'a self, f: &mut W, escaper: &impl Escaper) -> Result {
                #[cfg(feature = "debug-fast-escape-type-priority")]
                f.write_str("int(")?;

                escaper.escape(f, itoa::Buffer::new().format(*self))?;

                #[cfg(feature = "debug-fast-escape-type-priority")]
                f.write_str(")")?;

                Ok(())
            }

            #[inline]
            fn oxiplate_fast_raw(&'a self, f: &mut W) -> Result {
                #[cfg(feature = "debug-fast-escape-type-priority")]
                f.write_str("int(")?;

                f.write_str(itoa::Buffer::new().format(*self))?;

                #[cfg(feature = "debug-fast-escape-type-priority")]
                f.write_str(")")?;

                Ok(())
            }
        }
    )* };
}

unescaped_ints!(
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
);
