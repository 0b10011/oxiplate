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

#[test]
fn numbers() {
    assert_eq!(19, default(Some(19), 89));
    assert_eq!(89, default(None, 89));
}
