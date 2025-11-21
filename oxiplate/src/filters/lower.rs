use std::borrow::Cow;

use oxiplate_traits::CowStr;

/// Returns the lowercase version of the string.
///
/// ```
/// use std::fmt::Error;
///
/// use oxiplate::prelude::*;
///
/// #[derive(Oxiplate)]
/// #[oxiplate_inline(html: r#"{{ >"Hello World" | lower() }}"#)]
/// struct Data;
///
/// fn main() -> Result<(), Error> {
///     assert_eq!(Data.render()?, "hello world");
///     Ok(())
/// }
/// ```
pub fn lower<'a, E: CowStr<'a>>(expression: E) -> Cow<'a, str> {
    match expression.cow_str() {
        Cow::Borrowed(str) => str.to_lowercase().into(),
        Cow::Owned(string) => string.to_lowercase().into(),
    }
}
