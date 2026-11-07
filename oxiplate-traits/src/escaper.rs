use core::fmt::{Result, Write};

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

/// Trait to check if an item is safe for a specific escaper.
pub trait IsSafeFor {
    /// Escaper being checked against.
    type Escaper: Escaper;

    /// Check if an item is safe for the specified escaper.
    fn is_safe_for(&self, escaper: &Self::Escaper) -> bool;
}
