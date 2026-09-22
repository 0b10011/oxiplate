use quote::quote;

use crate::template::parser::Res;
use crate::template::parser::expression::arguments::{ArgumentsGroup, arguments};
use crate::template::parser::expression::{Expression, NestedExpression};
use crate::{BuiltTokens, Source, State, TokenSlice};

/// A call expression (`expr(args)`).
/// See: <https://doc.rust-lang.org/reference/expressions/call-expr.html>
#[derive(Debug)]
pub(crate) struct Call<'a> {
    pub(super) expression: Box<Expression<'a>>,
    arguments: ArgumentsGroup<'a>,
}

impl<'a> Call<'a> {
    /// Parses a call expression (`expr(args)`).
    /// See: <https://doc.rust-lang.org/reference/expressions/call-expr.html#grammar-CallExpression>
    pub(super) fn parser(tokens: TokenSlice<'a>) -> Res<'a, Box<NestedExpression<'a>>> {
        let (tokens, arguments) = arguments(tokens)?;

        Ok((
            tokens,
            Box::new(|expression: Expression<'a>| {
                Expression::Call(Self {
                    expression: Box::new(expression),
                    arguments,
                })
            }),
        ))
    }

    /// Generates token stream for entire call expression.
    pub(super) fn to_tokens(&self, state: &State) -> BuiltTokens {
        let (expression, _estimated_length) = self.expression.to_tokens(state);
        let arguments = self.arguments.to_tokens(state);

        match &*self.expression {
            Expression::Path(path) if path.template_field(state).is_some() => {
                (quote! { (#expression) #arguments }, 1)
            }
            _ => (quote! { #expression #arguments }, 1),
        }
    }

    /// Builds source for entire call expression.
    pub fn source(&self) -> Source<'a> {
        self.expression.source().merge(
            &self.arguments.source(),
            "Arguments expected after expression",
        )
    }
}
