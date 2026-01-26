use crate::syntax::parser::{Parser, TokenSlice};
use crate::syntax::{Error, Res};

/// Builds a parser that returns the first `Ok()` result
/// from the parsers in the tuple.
///
/// Runs the first parser in the tuple
/// and returns the result if it is `Ok()`,
/// otherwise repeats on each item in the tuple.
/// If only errors are found,
/// returns `Error::Multiple()` with errors
/// from each parser in the tuple.
///
/// ```rust,ignore
/// let (tokens, token) = alt((
///     take(TokenKind::StaticText),
///     take(TokenKind::StaticWhitespace),
/// ))
/// .parse(tokens)?;
/// ```
pub fn alt<P>(parsers: P) -> Alt<P> {
    Alt { parsers }
}

pub struct Alt<P> {
    parsers: P,
}

/// Implements `Parser` for `Alt<(P0, P1)>`, `Alt<(P0, P1, P2)>`, etc
/// until an implementation has been built for all provided members.
macro_rules! alt {
    // Add missing trailing comma
    ($id1:tt $generic1:ident, $id2:tt $generic2:ident) => {
        alt!($id1 $generic1, $id2 $generic2, );
    };

    // Split first two out to implement for two members
    ($id1:tt $generic1:ident, $id2:tt $generic2:ident, $($id:tt $generic:ident),*) => {
        alt!(impl $id1 $generic1, $id2 $generic2 | $($id $generic,)*);
    };

    // Implement for first group, move over one member, and repeat
    (impl $($id1:tt $generic1:ident),+ | $id2:tt $generic2:ident, $($id:tt $generic:ident,)+) => {
        // Implement for tokens before the `|`
        alt!(impl $($id1 $generic1),+);

        // Move tokens for the first member after the `|` to before the `|`
        // to implement for three-plus members (recursive)
        alt!(impl $($id1 $generic1,)+ $id2 $generic2 | $($id $generic,)+);
    };

    // Implement for the provided members
    (impl $($id:tt $generic:ident),+) => {
        impl<'a, O, $($generic),+> Parser<'a> for Alt<($($generic,)+)>
        where
            $($generic: Parser<'a, Output = O>),+
        {
            type Output = O;

            #[inline]
            fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
                let mut errors = vec![];
                $(
                    match self.parsers.$id.parse(tokens.clone()) {
                        result @ (Ok(_) | Err(Error::Unrecoverable { .. })) => return result,
                        Err(err) => errors.push(err),
                    }
                )+

                Err(Error::Multiple(errors))
            }
        }
    };

    // Handle final two groups.
    // The final group panics to give an easier to understand error message
    // when attempting to add another item to `alt()`.
    (impl $($id1:tt $generic1:ident),+ | $id2:tt $generic2:ident, ) => {
        // Implement for tokens before the `|`
        alt!(impl $($id1 $generic1),+);

        // Implement for all tokens
        impl<'a, O, $($generic1,)+ $generic2> Parser<'a> for Alt<($($generic1,)+ $generic2)>
        where
            $($generic1: Parser<'a, Output = O>,)+
            $generic2: Parser<'a, Output = O>
        {
            type Output = O;

            #[inline]
            fn parse(&self, _tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
                unimplemented!(
                    "Attempting to use `alt()` with {} variants, but only {} variants are currently supported. Consider adding additional member tokens to the `alt!()` invocation, or reducing the number of items passed into `alt()`.",
                    $id2 + 1,
                    $id2
                );
            }
        }
    };
}

alt!(
    0 P0, 1 P1, 2 P2, 3 P3, 4 P4, 5 P5, 6 P6, 7 P7, 8 P8, 9 P9,
    10 P10, 11 P11, 12 P12, 13 P13, 14 P14, 15 P15, 16 P16, 17 P17, 18 P18, 19 P19
);

#[test]
#[should_panic = "Attempting to use `alt()` with 20 variants, but only 19 variants are currently \
                  supported. Consider adding additional member tokens to the `alt!()` invocation, \
                  or reducing the number of items passed into `alt()`."]
fn max_alt_variants() {
    use crate::source::test_source;
    use crate::syntax::expression::KeywordParser;
    use crate::tokenizer::Eof;

    test_source!(source = "Hello world");

    alt((
        KeywordParser::new("keyword_1"),
        KeywordParser::new("keyword_2"),
        KeywordParser::new("keyword_3"),
        KeywordParser::new("keyword_4"),
        KeywordParser::new("keyword_5"),
        KeywordParser::new("keyword_6"),
        KeywordParser::new("keyword_7"),
        KeywordParser::new("keyword_8"),
        KeywordParser::new("keyword_9"),
        KeywordParser::new("keyword_10"),
        KeywordParser::new("keyword_11"),
        KeywordParser::new("keyword_12"),
        KeywordParser::new("keyword_13"),
        KeywordParser::new("keyword_14"),
        KeywordParser::new("keyword_15"),
        KeywordParser::new("keyword_16"),
        KeywordParser::new("keyword_17"),
        KeywordParser::new("keyword_18"),
        KeywordParser::new("keyword_19"),
        KeywordParser::new("keyword_20"),
    ))
    .parse(TokenSlice::new(&[], &Eof::for_test(source)))
    .unwrap();
}
