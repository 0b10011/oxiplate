extern crate alloc;

use alloc::borrow::Cow;

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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::string::String;
    use core::fmt::Display;

    use oxiplate_traits::{ToCowStr, cow_str_wrapper};

    #[test]
    fn str() {
        assert_eq!("HELLO ММ ЦЦ", super::upper(cow_str_wrapper!("Hello Мм Цц")));
    }

    #[test]
    fn string() {
        assert_eq!(
            "WORLD",
            super::upper(cow_str_wrapper!(String::from("World")))
        );
    }

    #[test]
    fn integer() {
        assert_eq!("19", super::upper(cow_str_wrapper!(19)));
    }

    #[test]
    fn display() {
        struct Data;
        impl Display for Data {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("Data")
            }
        }

        assert_eq!("DATA", super::upper(cow_str_wrapper!(Data)));
    }
}
