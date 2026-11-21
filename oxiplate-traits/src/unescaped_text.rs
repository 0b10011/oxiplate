//! Specialized text escape calls for more efficient conversions to `&str`.
//! See [`UnescapedTextWrapper`].

extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use core::fmt::{Display, Result, Write};

use crate::PreparedText;
use crate::escaper::Escaper;

/// Wrapper around unescaped text
/// that will implement [`crate::private::WriteEscaped`]
/// and [`crate::private::WriteRaw`]
/// via [`FastWriteEscaped`] and [`FastWriteRaw`] when possible,
/// otherwise via `Display`.
///
/// [`super::TextHandler`] may cause escaping to be skipped
/// for certain types.
///
/// Must borrow two times
/// before calling `oxiplate_write_escaped()` or `oxiplate_write_raw()`.
/// This is how the specialized implementations get called
/// in the correct order.
///
/// ```rust
/// # use oxiplate_traits as oxiplate;
/// use oxiplate::PreparedText;
/// use oxiplate::private::{UnescapedTextWrapper, WriteRaw as _};
/// let text = "hello world";
/// let mut string = String::new();
/// let formatter = &mut string;
/// (&&UnescapedTextWrapper::new(PreparedText::NeedsEscaped(&text)))
///     .oxiplate_write_raw(formatter)?;
/// assert_eq!("hello world", string);
/// # Ok::<(), ::std::fmt::Error>(())
/// ```
pub struct UnescapedTextWrapper<'a, T: ?Sized>(PreparedText<'a, T>);

impl<'a, T> UnescapedTextWrapper<'a, T> {
    /// Wrap unescaped text.
    #[must_use]
    pub fn new(prepared_text: PreparedText<'a, T>) -> Self {
        Self(prepared_text)
    }
}

/// Trait with specialized implementations
/// for items that implement `FastWriteEscaped` and `FastWriteRaw`,
/// traits that allow for more efficient writing.
///
/// See [`UnescapedTextWrapper`] documentation for more information.
pub trait WriteEscaped<'a: 'b, 'b, E, W>: WriteRaw<'a, 'b, W>
where
    E: Escaper,
    W: Write + ?Sized,
{
    /// Escape, when necessary, and write a value to a writer.
    /// Called from generated templates
    /// whenever an escaper is specified or set as the default.
    ///
    /// [`super::TextHandler`] may cause escaping to be skipped
    /// for certain types.
    ///
    /// See [`UnescapedTextWrapper`] documentation for more information.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn oxiplate_write_escaped(&'a self, f: &'b mut W, escaper: &E) -> Result;
}

/// Trait with specialized implementations
/// for items that implement `FastWriteRaw`,
/// a trait that allows for more efficient writing.
///
/// See [`UnescapedTextWrapper`] documentation for more information.
pub trait WriteRaw<'a: 'b, 'b, W>
where
    W: Write + ?Sized,
{
    /// Write a value to a writer.
    /// Called from generated templates whenever raw output is used.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn oxiplate_write_raw(&'a self, f: &'b mut W) -> Result;
}

impl<'a: 'b, 'b, E, T, W> WriteEscaped<'a, 'b, E, W> for &UnescapedTextWrapper<'a, T>
where
    E: Escaper,
    T: FastWriteEscaped<'a, E, W> + FastWriteRaw<'a, W>,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_write_escaped(&'a self, f: &'b mut W, escaper: &E) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("FastWriteEscaped(")?;

        match self.0 {
            PreparedText::NeedsEscaped(value) => value.oxiplate_fast_write_escaped(f, escaper)?,
            PreparedText::AlreadyEscaped(_) => self.oxiplate_write_raw(f)?,
        }

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a: 'b, 'b, T, W> WriteRaw<'a, 'b, W> for &UnescapedTextWrapper<'a, T>
where
    T: FastWriteRaw<'a, W>,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_write_raw(&'a self, f: &'b mut W) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("FastWriteRaw(")?;

        match self.0 {
            PreparedText::NeedsEscaped(value) | PreparedText::AlreadyEscaped(value) => {
                value.oxiplate_fast_write_raw(f)?;
            }
        }

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a: 'b, 'b, E, T, W> WriteEscaped<'a, 'b, E, W> for UnescapedTextWrapper<'a, T>
where
    E: Escaper,
    T: ToString + Display,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_write_escaped(&'a self, f: &'b mut W, escaper: &E) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("DisplayEscaped(")?;

        match self.0 {
            PreparedText::NeedsEscaped(value) => escaper.escape(f, &ToString::to_string(value))?,
            PreparedText::AlreadyEscaped(_) => self.oxiplate_write_raw(f)?,
        }

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a: 'b, 'b, T, W> WriteRaw<'a, 'b, W> for UnescapedTextWrapper<'a, T>
where
    T: ToString + Display,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_write_raw(&'a self, f: &'b mut W) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("DisplayRaw(")?;

        match self.0 {
            PreparedText::NeedsEscaped(value) | PreparedText::AlreadyEscaped(value) => {
                f.write_str(&ToString::to_string(value))?;
            }
        }

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

/// Trait that allows for more efficient writing of values with escaping.
pub trait FastWriteEscaped<'a, E, W>
where
    E: Escaper,
    W: Write + ?Sized,
{
    /// Efficiently escape and write a value to a writer.
    ///
    /// # Errors
    ///
    /// If the escaped string cannot be written to the writer.
    fn oxiplate_fast_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result;
}

/// Trait that allows for more efficient writing of values without escaping.
pub trait FastWriteRaw<'a, W>
where
    W: Write + ?Sized,
{
    /// Efficiently write a value as-is to a writer.
    ///
    /// # Errors
    ///
    /// If the string cannot be written to the writer.
    fn oxiplate_fast_write_raw(&'a self, f: &mut W) -> Result;
}

impl<'a, E, T, W> FastWriteEscaped<'a, E, W> for &T
where
    E: Escaper,
    T: FastWriteEscaped<'a, E, W> + ?Sized,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result {
        <T>::oxiplate_fast_write_escaped(self, f, escaper)
    }
}

impl<'a, T, W> FastWriteRaw<'a, W> for &T
where
    T: FastWriteRaw<'a, W> + ?Sized,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_raw(&'a self, f: &mut W) -> Result {
        <T>::oxiplate_fast_write_raw(self, f)
    }
}

impl<'a, E, T, W> FastWriteEscaped<'a, E, W> for &mut T
where
    E: Escaper,
    T: FastWriteEscaped<'a, E, W> + ?Sized,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result {
        <T>::oxiplate_fast_write_escaped(self, f, escaper)
    }
}

impl<'a, T, W> FastWriteRaw<'a, W> for &mut T
where
    T: FastWriteRaw<'a, W> + ?Sized,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_raw(&'a self, f: &mut W) -> Result {
        <T>::oxiplate_fast_write_raw(self, f)
    }
}

impl<'a, E, T, W> FastWriteEscaped<'a, E, W> for Box<T>
where
    E: Escaper,
    T: FastWriteEscaped<'a, E, W> + ?Sized,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result {
        <T>::oxiplate_fast_write_escaped(self.as_ref(), f, escaper)
    }
}

impl<'a, T, W> FastWriteRaw<'a, W> for Box<T>
where
    T: FastWriteRaw<'a, W> + ?Sized,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_raw(&'a self, f: &mut W) -> Result {
        <T>::oxiplate_fast_write_raw(self.as_ref(), f)
    }
}

impl<'a, E, T, W> FastWriteEscaped<'a, E, W> for Rc<T>
where
    E: Escaper,
    T: FastWriteEscaped<'a, E, W> + ?Sized,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result {
        <T>::oxiplate_fast_write_escaped(self.as_ref(), f, escaper)
    }
}

impl<'a, T, W> FastWriteRaw<'a, W> for Rc<T>
where
    T: FastWriteRaw<'a, W> + ?Sized,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_raw(&'a self, f: &mut W) -> Result {
        <T>::oxiplate_fast_write_raw(self.as_ref(), f)
    }
}

impl<'a, E, T, W> FastWriteEscaped<'a, E, W> for Arc<T>
where
    E: Escaper,
    T: FastWriteEscaped<'a, E, W> + ?Sized,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result {
        <T>::oxiplate_fast_write_escaped(self.as_ref(), f, escaper)
    }
}

impl<'a, T, W> FastWriteRaw<'a, W> for Arc<T>
where
    T: FastWriteRaw<'a, W> + ?Sized,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_raw(&'a self, f: &mut W) -> Result {
        <T>::oxiplate_fast_write_raw(self.as_ref(), f)
    }
}

impl<'a, E, W> FastWriteEscaped<'a, E, W> for String
where
    E: Escaper,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("String(")?;

        escaper.escape(f, self)?;

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, W> FastWriteRaw<'a, W> for String
where
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_raw(&'a self, f: &mut W) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("String(")?;

        f.write_str(self)?;

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, E, W> FastWriteEscaped<'a, E, W> for str
where
    E: Escaper,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("str(")?;

        escaper.escape(f, self)?;

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, W> FastWriteRaw<'a, W> for str
where
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_raw(&'a self, f: &mut W) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("str(")?;

        f.write_str(self)?;

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, E, W> FastWriteEscaped<'a, E, W> for bool
where
    E: Escaper,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("bool(")?;

        escaper.escape(f, if *self { "true" } else { "false" })?;

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, W> FastWriteRaw<'a, W> for bool
where
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_fast_write_raw(&'a self, f: &mut W) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("bool(")?;

        f.write_str(if *self { "true" } else { "false" })?;

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

#[cfg(feature = "fast-escape-ints")]
macro_rules! unescaped_ints {
    ($($ty:ty)*) => { $(
        impl<'a, E, W> FastWriteEscaped<'a, E, W> for $ty
        where
            E: Escaper,
            W: Write + ?Sized,
        {
            #[inline]
            fn oxiplate_fast_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result {
                #[cfg(feature = "_debug-fast-write-type-priority")]
                f.write_str("int(")?;

                escaper.escape(f, itoa::Buffer::new().format(*self))?;

                #[cfg(feature = "_debug-fast-write-type-priority")]
                f.write_str(")")?;

                Ok(())
            }
        }

        impl<'a, W> FastWriteRaw<'a, W> for $ty
        where
            W: Write + ?Sized,
        {
            #[inline]
            fn oxiplate_fast_write_raw(&'a self, f: &mut W) -> Result {
                #[cfg(feature = "_debug-fast-write-type-priority")]
                f.write_str("int(")?;

                f.write_str(itoa::Buffer::new().format(*self))?;

                #[cfg(feature = "_debug-fast-write-type-priority")]
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
