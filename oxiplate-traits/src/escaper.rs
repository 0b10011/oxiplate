use core::fmt::{Result, Write};

/// Trait for an Oxiplate-compatible escaper group.
pub trait Escaper
where
    Self: Sized,
{
    /// The default escaper for this escaper group.
    const DEFAULT: Self;

    /// Builder to build an item
    /// that implements `IsEscaped`
    /// for a particular value.
    type IsEscaped<'a, T: 'a>: IsEscaped<'a, Self, T>;

    /// Function that escapes text based on the selected variant.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result;

    /// Get the builder to build an item
    /// that implements `IsEscaped`
    /// for a particular value.
    fn build_is_escaped<T>(value: &T) -> Self::IsEscaped<'_, T>
    where
        Self: Sized;
}

/// Trait to check if a value is already escaped for a provided escaper.
pub trait IsEscaped<'a, E, T>
where
    E: Escaper,
{
    /// Whether the input is already considered escaped
    /// for the provided escaper.
    fn is_escaped(&'a self, escaper: &E) -> (&'a T, bool);
}
