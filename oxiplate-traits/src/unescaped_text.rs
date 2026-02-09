//! Specialized text escape calls for more efficient conversions to `&str`.
//! See [`UnescapedTextWrapper`].

extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use core::fmt::{Display, Result, Write};

use crate::escaper::Escaper;

/// Wrapper around unescaped text
/// that will implement `UnescapedText`
/// via `FastEscape` when possible,
/// otherwise via `Display`.
/// Must borrow twice before calling `oxiplate_escape()` or `oxiplate_raw()`.
///
/// ```rust
/// # use oxiplate_traits as oxiplate;
/// use oxiplate::UnescapedText;
/// let text = "hello world";
/// let mut string = String::new();
/// let formatter = &mut string;
/// (&&oxiplate::UnescapedTextWrapper::new(&text)).oxiplate_raw(formatter)?;
/// assert_eq!("hello world", string);
/// # Ok::<(), ::std::fmt::Error>(())
/// ```
pub struct UnescapedTextWrapper<'a, T: ?Sized>(&'a T);

impl<'a, T> UnescapedTextWrapper<'a, T> {
    /// Wrap unescaped text.
    pub fn new(value: &'a T) -> Self {
        Self(value)
    }
}

/// Trait with a specialized implementation
/// for items that implement `FastEscape`,
/// a trait that allows for more efficient conversions to `&str`.
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

        escaper.escape(f, &ToString::to_string(self.0))?;

        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str(")")?;

        Ok(())
    }

    #[inline]
    fn oxiplate_raw(&'a self, f: &mut W) -> Result {
        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str("Display(")?;

        f.write_str(&ToString::to_string(self.0))?;

        #[cfg(feature = "debug-fast-escape-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

/// Trait that allows for more efficient conversions to `&str`.
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

#[cfg(feature = "fast-escape-ints")]
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

#[cfg(feature = "fast-escape-ints")]
unescaped_ints!(
    i8 i16 i32 i64 i128 isize
    u8 u16 u32 u64 u128 usize
);
