mod alt;
mod context;
mod cut;
mod fail;
mod into;
mod many0;
mod many1;
mod opt;
mod parse_all;
mod take;

pub use alt::alt;
pub use context::context;
pub use cut::cut;
pub use fail::fail;
pub use into::into;
pub use many0::many0;
pub use many1::many1;
pub use opt::opt;
pub use parse_all::parse_all;
pub use take::take;

use crate::syntax::Res;
use crate::tokenizer::TokenSlice;

pub trait Parser<'a> {
    type Output;

    #[must_use]
    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output>;
}

impl<'a, O, F> Parser<'a> for F
where
    F: Fn(TokenSlice<'a>) -> Res<'a, O>,
{
    type Output = O;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        self(tokens)
    }
}

/// Implements `Parser` for `(P0, P1)`, `(P0, P1, P2)`, etc
/// until an implementation has been built for all provided members.
macro_rules! tuple {
    // Add missing trailing comma
    ($id1:tt $var1:ident $generic1:ident, $id2:tt $var2:ident $generic2:ident) => {
        tuple!($id1 $var1 $generic1, $id2 $var2 $generic2, );
    };

    // Split first two out to implement for two members
    ($id1:tt $var1:ident $generic1:ident, $id2:tt $var2:ident $generic2:ident, $($id:tt $var:ident $generic:ident),*) => {
        tuple!(impl $id1 $var1 $generic1, $id2 $var2 $generic2 | $($id $var $generic,)*);
    };

    // Implement for first group, move over one member, and repeat
    (impl $($id1:tt $var1:ident $generic1:ident),+ | $id2:tt $var2:ident $generic2:ident, $($id:tt $var:ident $generic:ident,)+) => {
        // Implement for tokens before the `|`
        tuple!(impl $($id1 $var1 $generic1),+);

        // Move tokens for the first member after the `|` to before the `|`
        // to implement for three-plus members (recursive)
        tuple!(impl $($id1 $var1 $generic1,)+ $id2 $var2 $generic2 | $($id $var $generic,)+);
    };

    // Implement for the provided members
    (impl $($id:tt $var:ident $generic:ident),+) => {
        impl<'a, $($generic),+> Parser<'a> for ($($generic,)+)
        where
            $($generic: Parser<'a>),+
        {
            type Output = ($(<$generic as Parser<'a>>::Output,)+);

            #[inline]
            fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
                $(let (tokens, $var) = self.$id.parse(tokens)?;)+

                Ok((tokens, ($($var,)+)))
            }
        }
    };

    // Handle final two groups.
    // The final group panics to give an easier to understand error message
    // when attempting to add another item to the tuple.
    (impl $($id1:tt $var1:ident $generic1:ident),+ | $id2:tt $var2:ident $generic2:ident, ) => {
        // Implement for tokens before the `|`
        tuple!(impl $($id1 $var1 $generic1),+);

        // Implement for all tokens
        impl<'a, $($generic1,)+ $generic2> Parser<'a> for ($($generic1,)+ $generic2)
        where
            $($generic1: Parser<'a>,)+
            $generic2: Parser<'a>
        {
            type Output = ($(<$generic1 as Parser<'a>>::Output,)+ <$generic2 as Parser<'a>>::Output);

            #[inline]
            fn parse(&self, _tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
                unimplemented!(
                    "Attempting to use a tuple with {} values for parsing, but only {} values are currently supported. Consider adding additional member tokens to the `tuple!()` invocation, or reducing the number of items in the tuple.",
                    $id2 + 1,
                    $id2
                );
            }
        }
    };
}

tuple!(
    0 p0 P0, 1 p1 P1, 2 p2 P2, 3 p3 P3, 4 p4 P4,
    5 p5 P5, 6 p6 P6, 7 p7 P7, 8 p8 P8, 9 p9 P9
);

#[test]
#[should_panic = "Attempting to use a tuple with 10 values for parsing, but only 9 values are \
                  currently supported. Consider adding additional member tokens to the `tuple!()` \
                  invocation, or reducing the number of items in the tuple."]
fn max_tuple_values() {
    use crate::source::test_source;
    use crate::syntax::expression::KeywordParser;
    use crate::tokenizer::Eof;

    test_source!(source = "Hello world");

    (
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
    )
        .parse(TokenSlice::new(&[], &Eof::for_test(source)))
        .unwrap();
}
