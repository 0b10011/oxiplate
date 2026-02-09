extern crate alloc;

use alloc::borrow::Cow;
#[cfg(test)]
use alloc::string::String;
#[cfg(test)]
use core::fmt::Display;

use oxiplate_traits::CowStr;
#[cfg(test)]
use oxiplate_traits::{cow_str_wrapper, ToCowStr};

/// Trims the leading and trailing whitespace from the input.
///
/// ```
/// use std::fmt::Error;
///
/// use oxiplate::prelude::*;
///
/// #[derive(Oxiplate)]
/// #[oxiplate_inline(html: r#"{{ >"  hello world   " | trim() }}"#)]
/// struct Data;
///
/// fn main() -> Result<(), Error> {
///     assert_eq!(Data.render()?, "hello world");
///     Ok(())
/// }
/// ```
pub fn trim<'a, E: CowStr<'a>>(expression: E) -> Cow<'a, str> {
    match expression.cow_str() {
        Cow::Borrowed(str) => str.trim().into(),
        Cow::Owned(string) => Cow::Owned(string.trim().into()),
    }
}

/// Trims the leading whitespace from the input.
///
/// ```
/// use std::fmt::Error;
///
/// use oxiplate::prelude::*;
///
/// #[derive(Oxiplate)]
/// #[oxiplate_inline(html: r#"{{ >"  hello world   " | trim_start() }}"#)]
/// struct Data;
///
/// fn main() -> Result<(), Error> {
///     assert_eq!(Data.render()?, "hello world   ");
///     Ok(())
/// }
/// ```
pub fn trim_start<'a, E: CowStr<'a>>(expression: E) -> Cow<'a, str> {
    match expression.cow_str() {
        Cow::Borrowed(str) => str.trim_start().into(),
        Cow::Owned(string) => Cow::Owned(string.trim_start().into()),
    }
}
/// Trims the trailing whitespace from the input.
///
/// ```
/// use std::fmt::Error;
///
/// use oxiplate::prelude::*;
///
/// #[derive(Oxiplate)]
/// #[oxiplate_inline(html: r#"{{ >"  hello world   " | trim_end() }}"#)]
/// struct Data;
///
/// fn main() -> Result<(), Error> {
///     assert_eq!(Data.render()?, "  hello world");
///     Ok(())
/// }
/// ```
pub fn trim_end<'a, E: CowStr<'a>>(expression: E) -> Cow<'a, str> {
    match expression.cow_str() {
        Cow::Borrowed(str) => str.trim_end().into(),
        Cow::Owned(string) => Cow::Owned(string.trim_end().into()),
    }
}

macro_rules! test {
    ($test_fn:ident, $input:literal, $trim_fn:ident, $expected:literal) => {
        #[test]
        fn $test_fn() {
            assert_eq!(
                $expected,
                $trim_fn(cow_str_wrapper!($input)),
                "testing string slice"
            );
            assert_eq!(
                $expected,
                $trim_fn(cow_str_wrapper!(String::from($input))),
                "testing `String`"
            );

            struct Data;
            impl Display for Data {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_str($input)
                }
            }
            assert_eq!(
                $expected,
                $trim_fn(cow_str_wrapper!(Data)),
                "testing `Display`"
            );
        }
    };
}

test!(test_trim, "\t\n Hello world!\t \n", trim, "Hello world!");
test!(
    test_trim_start,
    "\t\n Hello world!\t \n",
    trim_start,
    "Hello world!\t \n"
);
test!(
    test_trim_end,
    "\t\n Hello world!\t \n",
    trim_end,
    "\t\n Hello world!"
);
