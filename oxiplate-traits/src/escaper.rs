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
    type TextHandler<'a, T: 'a>: TextHandler<'a, Self, T>;

    /// Function that escapes text based on the selected variant.
    ///
    /// # Errors
    ///
    /// If escaped string cannot be written to the writer.
    fn escape<W: Write + ?Sized>(&self, f: &mut W, value: &str) -> Result;

    /// Get the builder to build an item
    /// that implements `IsEscaped`
    /// for a particular value.
    fn build_text_handler<T>(value: &T) -> Self::TextHandler<'_, T>
    where
        Self: Sized;
}

/// Wrapper around text to output in a template
/// that marks it as already escaped or needing escaped.
/// Used by [`crate::private::UnescapedTextWrapper`].
pub enum PreparedText<'a, T: ?Sized> {
    /// Value that needs to be escaped
    /// for the template it was generated for.
    NeedsEscaped(&'a T),

    /// Value that is already escaped
    /// for the template it was generated for.
    AlreadyEscaped(&'a T),
}

/// Trait to check if a value is already escaped for a provided escaper.
pub trait TextHandler<'a, E, T>
where
    E: Escaper,
{
    /// Whether the input is already considered escaped
    /// for the provided escaper.
    fn prepare_text_for_output(&'a self, escaper: &E) -> PreparedText<'a, T>;
}
