extern crate alloc;

use alloc::borrow::Cow;
#[cfg(test)]
use alloc::string::String;
#[cfg(test)]
use core::fmt::Display;

use oxiplate_traits::CowStr;
#[cfg(test)]
use oxiplate_traits::{cow_str_wrapper, ToCowStr};

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

#[test]
fn str() {
    assert_eq!("hello мм цц", lower(cow_str_wrapper!("Hello Мм Цц")));
}

#[test]
fn string() {
    assert_eq!("world", lower(cow_str_wrapper!(String::from("World"))));
}

#[test]
fn integer() {
    assert_eq!("19", lower(cow_str_wrapper!(19)));
}

#[test]
fn display() {
    struct Data;
    impl Display for Data {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("Data")
        }
    }

    assert_eq!("data", lower(cow_str_wrapper!(Data)));
}
