use std::borrow::Cow;

use oxiplate_traits::CowStr;

/// Returns the uppercase version of the string.
///
/// ```
/// use std::fmt::Error;
///
/// use oxiplate::prelude::*;
///
/// #[derive(Oxiplate)]
/// #[oxiplate_inline(html: r#"{{ >"Hello World" | upper() }}"#)]
/// struct Data;
///
/// fn main() -> Result<(), Error> {
///     assert_eq!(Data.render()?, "HELLO WORLD");
///     Ok(())
/// }
/// ```
pub fn upper<'a, E: CowStr<'a>>(expression: E) -> Cow<'a, str> {
    match expression.cow_str() {
        Cow::Borrowed(str) => str.to_uppercase().into(),
        Cow::Owned(string) => string.to_uppercase().into(),
    }
}

#[test]
fn str() {
    assert_eq!(
        "WORLD",
        upper(crate::CowStrWrapper::new(
            (&&crate::ToCowStrWrapper::new(&"world")).to_cow_str(),
        )),
    );
}
