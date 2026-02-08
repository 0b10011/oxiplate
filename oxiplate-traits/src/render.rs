extern crate alloc;

use alloc::string::String;
use core::fmt::{self, Error, Write};

/// Optimized render function trait.
pub trait Render {
    /// Estimated output length of the template.
    const ESTIMATED_LENGTH: usize;

    /// Render the template into a string.
    ///
    /// # Errors
    ///
    /// If strings cannot be written to the formatter.
    fn render(&self) -> Result<String, Error> {
        let mut string = String::with_capacity(Self::ESTIMATED_LENGTH);
        self.render_into(&mut string)?;
        Ok(string)
    }

    /// Render the template into a writer.
    ///
    /// # Errors
    ///
    /// If strings cannot be written to the formatter.
    fn render_into<W: Write>(&self, writer: &mut W) -> fmt::Result;
}
