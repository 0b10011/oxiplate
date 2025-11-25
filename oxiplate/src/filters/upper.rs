use std::borrow::Cow;
#[cfg(test)]
use std::fmt::Display;

use oxiplate_traits::CowStr;
#[cfg(test)]
use oxiplate_traits::{ToCowStr, cow_str_wrapper};

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
    assert_eq!("HELLO ММ ЦЦ", upper(cow_str_wrapper!("Hello Мм Цц")));
}

#[test]
fn string() {
    assert_eq!("WORLD", upper(cow_str_wrapper!(String::from("World"))));
}

#[test]
fn integer() {
    assert_eq!("19", upper(cow_str_wrapper!(19)));
}

#[test]
fn display() {
    struct Data;
    impl Display for Data {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("Data")
        }
    }

    assert_eq!("DATA", upper(cow_str_wrapper!(Data)));
}
