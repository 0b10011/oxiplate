extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::String;

use oxiplate_traits::CowStr;
use unicode_segmentation::UnicodeSegmentation;

/// Shortens the string to `max_length` grapheme clusters or fewer,
/// ending with `…` when shortened.
/// Avoids cutting words in half
/// unless the first word alone is too long to fit.
///
/// Requires the `filter-shorten` feature.
///
/// ```
/// use std::fmt::Error;
///
/// use oxiplate::prelude::*;
///
/// #[derive(Oxiplate)]
/// #[oxiplate_inline(html: r#"{{ >"Hello wonderful world" | shorten(12) }}"#)]
/// struct Data;
///
/// fn main() -> Result<(), Error> {
///     assert_eq!(Data.render()?, "Hello…");
///     Ok(())
/// }
/// ```
pub fn shorten<'a, E: CowStr<'a>>(expression: E, max_length: usize) -> Cow<'a, str> {
    if max_length == 0 {
        return Cow::Borrowed("");
    }

    let expression = expression.cow_str();

    let Some(length) = shortened_length(&expression, max_length) else {
        return expression;
    };

    match expression {
        Cow::Borrowed(str) => {
            let mut string = String::with_capacity(length + '…'.len_utf8());
            string.push_str(&str[..length]);
            string.push('…');
            string.into()
        }
        Cow::Owned(mut string) => {
            string.truncate(length);
            string.push('…');
            string.into()
        }
    }
}

/// Returns the byte length of the text to keep before the `…`,
/// or `None` if the string already fits.
fn shortened_length(str: &str, max_length: usize) -> Option<usize> {
    let mut graphemes = str.grapheme_indices(true);

    let (cut, _) = graphemes.nth(max_length - 1)?;
    graphemes.next()?;

    let kept = str[..cut].trim_end();
    if str[..cut].ends_with(char::is_whitespace) || str[cut..].starts_with(char::is_whitespace) {
        return Some(kept.len());
    }

    let Some(last_whitespace) = kept.rfind(char::is_whitespace) else {
        return Some(kept.len());
    };
    let before_last_word = kept[..last_whitespace].trim_end();
    if before_last_word.is_empty() {
        Some(kept.len())
    } else {
        Some(before_last_word.len())
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::borrow::Cow;
    use alloc::string::String;
    use core::fmt::Display;

    use oxiplate_traits::{ToCowStr, cow_str_wrapper};

    macro_rules! test {
        ($test_fn:ident, $input:expr, $max_length:literal, $expected:expr) => {
            #[test]
            fn $test_fn() {
                struct Data;
                impl Display for Data {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        f.write_str($input)
                    }
                }

                assert_eq!(
                    $expected,
                    super::shorten(cow_str_wrapper!($input), $max_length),
                    "testing string slice"
                );
                assert_eq!(
                    $expected,
                    super::shorten(cow_str_wrapper!(String::from($input)), $max_length),
                    "testing `String`"
                );

                assert_eq!(
                    $expected,
                    super::shorten(cow_str_wrapper!(Data), $max_length),
                    "testing `Display`"
                );
            }
        };
    }

    test!(shorter, "Hello world", 20, "Hello world");
    test!(exact, "Hello world", 11, "Hello world");
    test!(empty, "", 5, "");
    test!(word_boundary, "Hello wonderful world", 12, "Hello…");
    test!(
        complete_word,
        "Hello wonderful world",
        16,
        "Hello wonderful…"
    );
    test!(word_start, "aa bb cc dd", 7, "aa bb…");
    test!(word_start_after_spaces, "aa bb   cc dd", 9, "aa bb…");
    test!(word_start_multi_byte, "héé wörd ünïcode", 10, "héé wörd…");
    test!(trailing_whitespace, "Hello   wonderful world", 9, "Hello…");
    test!(long_word, "Supercalifragilistic", 6, "Super…");
    test!(long_first_word, "Supercalifragilistic word", 6, "Super…");
    test!(zero, "Hello world", 0, "");
    test!(zero_empty, "", 0, "");
    test!(one, "Hello world", 1, "…");
    test!(one_exact, "H", 1, "H");
    test!(
        combining_character_unchanged,
        "Cafe\u{301} ole\u{301}",
        8,
        "Cafe\u{301} ole\u{301}"
    );
    test!(
        combining_character,
        "Cafe\u{301}\u{301}tiere",
        5,
        "Cafe\u{301}\u{301}…"
    );
    test!(
        emoji_zwj_sequence,
        "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}abc",
        3,
        "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}…"
    );
    test!(
        emoji_zwj_sequence_unchanged,
        "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}",
        2,
        "\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F467}"
    );

    #[test]
    fn unchanged_stays_borrowed() {
        assert!(matches!(
            super::shorten(cow_str_wrapper!("Hello world"), 11),
            Cow::Borrowed("Hello world")
        ));
    }
}
