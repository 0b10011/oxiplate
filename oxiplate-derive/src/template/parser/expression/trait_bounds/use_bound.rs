use super::use_bound_generic_args::UseBoundGenericArgs;
use crate::template::parser::expression::KeywordParser;
use crate::template::parser::prelude::*;

/// Between `first_bounds` and `last_bound`,
/// there should always be at least one bound.
///
/// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-UseBound>
#[derive(Debug)]
pub(crate) struct UseBound<'a> {
    r#use: Source<'a>,
    use_bound_generic_args: UseBoundGenericArgs<'a>,
}

impl<'a> UseBound<'a> {
    /// See: <https://doc.rust-lang.org/reference/trait-bounds.html#railroad-UseBound>
    pub(crate) fn parse(tokens: TokenSlice<'a>) -> Res<'a, Self> {
        let (tokens, (r#use, use_bound_generic_args)) =
            (KeywordParser::new("use"), UseBoundGenericArgs::parse).parse(tokens)?;

        let r#use = r#use.source().clone();

        Ok((
            tokens,
            Self {
                r#use,
                use_bound_generic_args,
            },
        ))
    }

    pub(crate) fn source(&self) -> Source<'a> {
        self.r#use.clone().merge(
            self.use_bound_generic_args.source(),
            "Use bound generic args expected after `use`",
        )
    }
}

impl<'a> ToTokensWithState<'a> for UseBound<'a> {
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens {
        let span = self.r#use.span_token();
        let (use_bound_generic_args, estimated_length) =
            self.use_bound_generic_args.to_tokens_with_state(state);

        (
            quote_spanned! {span=> use #use_bound_generic_args },
            estimated_length,
        )
    }
}
