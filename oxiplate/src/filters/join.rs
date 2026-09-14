extern crate alloc;

use alloc::string::String;

use oxiplate_traits::CowStr;
#[cfg(test)]
use oxiplate_traits::{ToCowStr, cow_str_wrapper};

/// Joins an iterable of strings using the provided glue.
///
/// ```
/// use std::fmt::Error;
///
/// use oxiplate::prelude::*;
///
/// #[derive(Oxiplate)]
/// #[oxiplate_inline(html: r#"{{ values | join(>", ") }}"#)]
/// struct Data {
///     values: [&'static str; 3],
/// }
///
/// fn main() -> Result<(), Error> {
///     let data = Data {
///         values: ["red", "green", "blue"],
///     };
///     assert_eq!(data.render()?, "red, green, blue");
///     Ok(())
/// }
/// ```
pub fn join<'a, I, S, G>(expression: I, glue: G) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
    G: CowStr<'a>,
{
    let glue = glue.cow_str();
    let mut values = expression.into_iter();
    let Some(first) = values.next() else {
        return String::new();
    };

    let mut joined = String::from(first.as_ref());
    for value in values {
        joined.push_str(&glue);
        joined.push_str(value.as_ref());
    }

    joined
}

#[test]
fn string_slices() {
    assert_eq!(
        "red, green, blue",
        join(["red", "green", "blue"], cow_str_wrapper!(", "))
    );
}

#[test]
fn strings() {
    assert_eq!(
        "red | green | blue",
        join(
            [
                String::from("red"),
                String::from("green"),
                String::from("blue"),
            ],
            cow_str_wrapper!(String::from(" | ")),
        )
    );
}

#[test]
fn empty() {
    assert_eq!("", join::<_, &str, _>([], cow_str_wrapper!(", ")));
}

#[test]
fn one() {
    assert_eq!("red", join(["red"], cow_str_wrapper!(", ")));
}
