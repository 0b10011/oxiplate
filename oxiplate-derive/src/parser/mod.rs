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

use std::fmt::Debug;

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

use crate::Source;
use crate::tokenizer::{TokenSlice, UnexpectedTokenError};

pub type Res<'a, K, S> = Result<(TokenSlice<'a, K>, S), Error<'a>>;

#[derive(Debug)]
pub enum Error<'a> {
    Recoverable {
        message: String,
        source: Source<'a>,
        #[allow(dead_code)]
        previous_error: Option<Box<Self>>,
        is_eof: bool,
    },
    Unrecoverable {
        message: String,
        source: Source<'a>,
        #[allow(dead_code)]
        previous_error: Option<Box<Self>>,
        is_eof: bool,
    },
    Multiple(Vec<Self>),
}

impl<'a> From<UnexpectedTokenError<'a>> for Error<'a> {
    fn from(value: UnexpectedTokenError<'a>) -> Self {
        let is_eof = value.is_eof();
        if is_eof {
            Self::Recoverable {
                message: value.message().to_string(),
                source: value.source().clone(),
                previous_error: None,
                is_eof,
            }
        } else {
            Self::Unrecoverable {
                message: value.message().to_string(),
                source: value.source().clone(),
                previous_error: None,
                is_eof,
            }
        }
    }
}

pub trait Parser<'a, K: Debug + PartialEq + Eq> {
    type Output;

    #[must_use]
    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output>;
}

impl<'a, K: Debug + PartialEq + Eq + 'a, O, F> Parser<'a, K> for F
where
    F: Fn(TokenSlice<'a, K>) -> Res<'a, K, O>,
{
    type Output = O;

    fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
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
        impl<'a, K, $($generic),+> Parser<'a, K> for ($($generic,)+)
        where
            K: Debug + PartialEq + Eq,
            $($generic: Parser<'a, K>),+
        {
            type Output = ($(<$generic as Parser<'a, K>>::Output,)+);

            #[inline]
            fn parse(&self, tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
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
        impl<'a, K, $($generic1,)+ $generic2> Parser<'a, K> for ($($generic1,)+ $generic2)
        where
            K: Debug + PartialEq + Eq,
            $($generic1: Parser<'a, K>,)+
            $generic2: Parser<'a, K>
        {
            type Output = ($(<$generic1 as Parser<'a, K>>::Output,)+ <$generic2 as Parser<'a, K>>::Output);

            #[inline]
            fn parse(&self, _tokens: TokenSlice<'a, K>) -> Res<'a, K, Self::Output> {
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
    use crate::template::TokenKind;
    use crate::tokenizer::Eof;

    test_source!(source = "Hello world");

    (
        take(TokenKind::Plus),
        take(TokenKind::Minus),
        take(TokenKind::Asterisk),
        take(TokenKind::ForwardSlash),
        take(TokenKind::Percent),
        take(TokenKind::Tilde),
        take(TokenKind::Comma),
        take(TokenKind::Ampersand),
        take(TokenKind::Exclamation),
        take(TokenKind::Period),
    )
        .parse(TokenSlice::new(&[], &Eof::for_test(source)))
        .unwrap();
}
