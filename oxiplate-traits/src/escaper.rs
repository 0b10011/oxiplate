use core::fmt::{Result, Write};

/// Trait for an Oxiplate-compatible escaper group.
pub trait Escaper {
    /// The default escaper for this escaper group.
    const DEFAULT: Self;

    /// Builder to build an item
    /// that implements `IsEscaped`
    /// for a particular value.
    type Builder<'a, T: 'a>: IsSafeForBuilder<'a, T>
    where
        Self: 'a;

    /// Function that escapes text based on the selected variant.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result;

    /// Get the builder to build an item
    /// that implements `IsEscaped`
    /// for a particular value.
    fn is_safe_for_builder<'a, T>() -> Self::Builder<'a, T>;
}

/// Trait to build an item
/// that implements `IsEscaped`
/// for a particular value.
pub trait IsSafeForBuilder<'a, T> {
    /// Item that implements `IsEscaped` for a particular value.
    type IsEscaped: IsEscaped;

    /// Build an item that implements `IsEscaped` for the provided value.
    fn build(&'a self, value: &'a T) -> Self::IsEscaped;
}

/// Trait to check if a value is already escaped for a provided escaper.
pub trait IsEscaped {
    /// Escaper to check if the input is escaped for.
    type Escaper: Escaper;

    /// Whether the input is already considered escaped
    /// for the provided escaper.
    fn is_escaped(&self, escaper: &Self::Escaper) -> bool;
}
