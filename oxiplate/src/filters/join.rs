extern crate alloc;

use alloc::string::String;

/// Joins an iterable of strings using the provided glue.
///
/// ```
/// use std::fmt::Error;
///
/// use oxiplate::prelude::*;
///
/// #[derive(Oxiplate)]
/// #[oxiplate_inline(html: r#"{{ values | join(", ") }}"#)]
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
pub fn join<I, S>(expression: I, glue: &str) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut values = expression.into_iter();
    let Some(first) = values.next() else {
        return String::new();
    };

    let mut joined = String::from(first.as_ref());
    for value in values {
        joined.push_str(glue);
        joined.push_str(value.as_ref());
    }

    joined
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::string::String;
    use alloc::vec;

    #[test]
    fn string_slices() {
        assert_eq!(
            "red, green, blue",
            super::join(["red", "green", "blue"], ", ")
        );
    }

    #[test]
    fn vec_of_string_slices() {
        assert_eq!(
            "red, green, blue",
            super::join(vec!["red", "green", "blue"], ", ")
        );
    }

    #[test]
    fn borrowed_vec_of_strings() {
        let values = vec![
            String::from("red"),
            String::from("green"),
            String::from("blue"),
        ];

        assert_eq!("red | green | blue", super::join(&values, " | "));
    }

    #[test]
    fn empty() {
        assert_eq!("", super::join::<_, &str>([], ", "));
    }

    #[test]
    fn one() {
        assert_eq!("red", super::join(["red"], ", "));
    }

    #[test]
    fn iterator() {
        let values = ["red", "green", "blue"].into_iter().map(str::to_uppercase);

        assert_eq!("RED/GREEN/BLUE", super::join(values, "/"));
    }
}
