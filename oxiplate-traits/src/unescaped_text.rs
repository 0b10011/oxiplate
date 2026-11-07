//! Specialized text escape calls for more efficient conversions to `&str`.
//! See [`UnescapedTextWrapper`].

extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use core::fmt::{Display, Result, Write};

use crate::IsSafeFor;
use crate::escaper::Escaper;

/// Wrapper around unescaped text
/// that will implement `WriteEscaped` and `WriteRaw`.
/// `FastWriteEscaped` or `FastWriteRaw` will be used when possible,
/// otherwise `Display` will be used.
/// `IsSafeFor` can cause escaping to be skipped entirely.
///
/// Must borrow four times
/// before calling `oxiplate_escape()` or `oxiplate_raw()`.
/// This is how the specialized implementations get called.
///
/// ```rust
/// # use oxiplate_traits as oxiplate;
/// use oxiplate::{FastWriteEscaped as _, FastWriteRaw as _, UnescapedTextWrapper};
/// let text = "hello world";
/// let mut string = String::new();
/// let formatter = &mut string;
/// (&&&&oxiplate::UnescapedTextWrapper::new(&text)).oxiplate_raw(formatter)?;
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

/// Trait with specialized implementations
/// for items that implement traits
/// that improve the performance of writing and/or escaping,
/// or make it possible to safely skip escaping entirely:
///
/// - `FastWriteEscaped` improves escaping and writing performance.
/// - `FastWriteRaw` improves writing performance when not escaping.
/// - `IsSafeFor` marks values as not needing escaping.
///
/// See `UnescapedTextWrapper` documentation for more information.
pub trait WriteEscaped<'a, E, W>: WriteRaw<'a, W>
where
    E: Escaper,
    W: Write + ?Sized,
{
    /// Escape and write a value to a writer.
    /// Called from generated templates whenever an escaper is used.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn oxiplate_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result;
}

/// Trait with specialized implementations
/// for items that implement `FastWriteRaw`
/// that improve the performance of writing
/// or make it possible to safely skip escaping entirely.
pub trait WriteRaw<'a, W>
where
    W: Write + ?Sized,
{
    /// Write a value to a writer.
    /// Called from generated templates whenever raw output is used.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn oxiplate_write_raw(&'a self, f: &mut W) -> Result;
}

impl<'a, E, T, W> WriteEscaped<'a, E, W> for &&&UnescapedTextWrapper<'a, T>
where
    E: Escaper,
    T: FastWriteEscaped<'a, E, W> + FastWriteRaw<'a, W> + IsSafeFor<Escaper = E>,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("FastWriteEscaped(")?;

        if self.0.is_safe_for(escaper) {
            #[cfg(feature = "_debug-fast-write-type-priority")]
            f.write_str("IsSafe(")?;

            self.0.oxiplate_fast_write_raw(f)?;

            #[cfg(feature = "_debug-fast-write-type-priority")]
            f.write_str(")")?;
        } else {
            #[cfg(feature = "_debug-fast-write-type-priority")]
            f.write_str("NotSafe(")?;

            self.0.oxiplate_fast_write_escaped(f, escaper)?;

            #[cfg(feature = "_debug-fast-write-type-priority")]
            f.write_str(")")?;
        }

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, T, W> WriteRaw<'a, W> for &&&UnescapedTextWrapper<'a, T>
where
    T: FastWriteRaw<'a, W>,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_write_raw(&'a self, f: &mut W) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("FastWriteEscaped(")?;

        self.0.oxiplate_fast_write_raw(f)?;

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, E, T, W> WriteEscaped<'a, E, W> for &&UnescapedTextWrapper<'a, T>
where
    E: Escaper,
    T: FastWriteEscaped<'a, E, W> + FastWriteRaw<'a, W>,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("FastWriteEscaped(")?;

        self.0.oxiplate_fast_write_escaped(f, escaper)?;

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, T, W> WriteRaw<'a, W> for &&UnescapedTextWrapper<'a, T>
where
    T: FastWriteRaw<'a, W>,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_write_raw(&'a self, f: &mut W) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("FastWriteEscaped(")?;

        self.0.oxiplate_fast_write_raw(f)?;

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, E, T, W> WriteEscaped<'a, E, W> for &UnescapedTextWrapper<'a, T>
where
    E: Escaper,
    T: ToString + Display + IsSafeFor<Escaper = E>,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("Display(")?;

        if self.0.is_safe_for(escaper) {
            #[cfg(feature = "_debug-fast-write-type-priority")]
            f.write_str("IsSafe(")?;

            f.write_str(&ToString::to_string(self.0))?;

            #[cfg(feature = "_debug-fast-write-type-priority")]
            f.write_str(")")?;
        } else {
            #[cfg(feature = "_debug-fast-write-type-priority")]
            f.write_str("NotSafe(")?;

            escaper.escape(f, &ToString::to_string(self.0))?;

            #[cfg(feature = "_debug-fast-write-type-priority")]
            f.write_str(")")?;
        }

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, T, W> WriteRaw<'a, W> for &UnescapedTextWrapper<'a, T>
where
    T: ToString + Display,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_write_raw(&'a self, f: &mut W) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("Display(")?;

        f.write_str(&ToString::to_string(self.0))?;

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, E, T, W> WriteEscaped<'a, E, W> for UnescapedTextWrapper<'a, T>
where
    E: Escaper,
    T: ToString + Display,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_write_escaped(&'a self, f: &mut W, escaper: &E) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("Display(")?;

        escaper.escape(f, &ToString::to_string(self.0))?;

        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str(")")?;

        Ok(())
    }
}

impl<'a, T, W> WriteRaw<'a, W> for UnescapedTextWrapper<'a, T>
where
    T: ToString + Display,
    W: Write + ?Sized,
{
    #[inline]
    fn oxiplate_write_raw(&'a self, f: &mut W) -> Result {
        #[cfg(feature = "_debug-fast-write-type-priority")]
        f.write_str("Display(")?;

        f.write_str(&ToString::to_string(self.0))?;

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
