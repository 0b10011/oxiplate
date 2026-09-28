/// Returns the `Some` value or the provided default.
///
/// ```
/// use std::fmt::Error;
///
/// use oxiplate::prelude::*;
///
/// #[derive(Oxiplate)]
/// #[oxiplate_inline(html: r#"{{ value | default("Bar") }}"#)]
/// struct Data {
///     value: Option<&'static str>,
/// }
///
/// fn main() -> Result<(), Error> {
///     assert_eq!(Data { value: None }.render()?, "Bar");
///     Ok(())
/// }
/// ```
pub fn default<T>(expression: Option<T>, default: T) -> T {
    expression.unwrap_or(default)
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    #[test]
    fn numbers() {
        assert_eq!(19, super::default(Some(19), 89));
        assert_eq!(89, super::default(None, 89));
    }
}
