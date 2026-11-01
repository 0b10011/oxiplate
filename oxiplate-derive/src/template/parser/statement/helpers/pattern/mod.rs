mod array;
mod bind;
mod literal;
mod range;
mod r#struct;
mod tuple;

use std::collections::HashSet;

use self::array::Array;
use self::bind::Bind;
use self::literal::Literal;
use self::range::Range;
use self::r#struct::StructPattern;
use self::tuple::Tuple;
use crate::template::parser::expression::Identifier;
use crate::template::parser::prelude::*;

#[derive(Debug)]
pub(crate) enum Pattern<'a> {
    Literal(Literal<'a>),
    Ident(Identifier<'a>),
    Range(Range<'a>),
    Struct(Box<StructPattern<'a>>),
    Array(Array<'a>),
    Tuple(Tuple<'a>),
    Bind(Bind<'a>),
}

impl<'a> Pattern<'a> {
    #[cfg(test)]
    pub fn new_for_test(source: &'a Source<'a>) -> Self {
        use crate::template::parser::expression::Integer;

        Self::Literal(Literal::Integer(Integer::new_for_test(source)))
    }

    pub(in crate::template::parser) fn parser() -> PatternParser<'a> {
        PatternParser {
            phantom_data: PhantomData,
        }
    }

    pub fn source(&self) -> Source<'a> {
        match self {
            Self::Literal(literal) => literal.source().to_owned(),
            Self::Ident(identifier) => identifier.source().to_owned(),
            Self::Bind(bind) => bind.source().to_owned(),
            Self::Range(range) => range.source().to_owned(),
            Self::Struct(r#struct) => r#struct.source(),
            Self::Array(array) => array.source().to_owned(),
            Self::Tuple(tuple) => tuple.source().to_owned(),
        }
    }

    pub fn get_variables(&'a self) -> HashSet<&'a str> {
        match self {
            Self::Ident(identifier) => HashSet::from([identifier.as_str()]),
            Self::Bind(value) => value.get_variables(),
            Self::Struct(value) => value.get_variables(),
            Self::Array(value) => value.get_variables(),
            Self::Tuple(value) => value.get_variables(),
            Self::Literal(_) | Self::Range(_) => HashSet::new(),
        }
    }
}

impl<'a> ToTokensWithState<'a> for Pattern<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        match self {
            Self::Literal(value) => value.to_tokens_with_state(state),
            Self::Ident(value) => (quote! { #value }, EstimatedLength::new(1)),
            Self::Bind(value) => value.to_tokens_with_state(state),
            Self::Range(value) => value.to_tokens_with_state(state),
            Self::Struct(value) => value.to_tokens_with_state(state),
            Self::Array(value) => value.to_tokens_with_state(state),
            Self::Tuple(value) => value.to_tokens_with_state(state),
        }
    }
}

impl<'a> From<Identifier<'a>> for Pattern<'a> {
    fn from(value: Identifier<'a>) -> Self {
        Pattern::Ident(value)
    }
}

pub(in crate::template::parser) struct PatternParser<'a> {
    phantom_data: PhantomData<&'a ()>,
}

impl<'a> Parser<'a> for PatternParser<'a> {
    type TokenKind = TokenKind;
    type Output = Pattern<'a>;

    fn parse(&self, tokens: TokenSlice<'a>) -> Res<'a, Self::Output> {
        alt((
            into(Range::parser()),
            into(Literal::parser()),
            into(Array::parser()),
            into(Tuple::parser()),
            into(StructPattern::parser()),
            into(Bind::parser()),
            into(Identifier::parse_prevent_reserved_keywords()),
        ))
        .parse(tokens)
    }
}
