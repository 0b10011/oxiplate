mod comment;
mod expression;
mod item;
mod statement;
mod r#static;
mod template;
mod writ;

use std::ops::{Add, AddAssign, Mul};

use self::item::Item;
use self::prelude::*;
use self::statement::Statement;
use self::r#static::Static;
use self::template::Template;
pub(crate) use self::template::parse;
use self::writ::Writ;

type Res<'a, S> = crate::parser::Res<'a, TokenKind, S>;

mod prelude {
    pub(super) use proc_macro2::TokenStream;
    pub(super) use quote::{ToTokens, TokenStreamExt, quote, quote_spanned};

    pub(super) use super::{EstimatedLength, Res, ToTokensWithMutState, ToTokensWithState};
    pub(super) use crate::parser::prelude::*;
    pub(super) use crate::template::tokenizer::TokenKind;
    pub(super) use crate::{BuiltTokens, Source, State, TokenSlice, internal_error};
}

impl<'a> Error<'a> {
    pub fn source(&self) -> &Source<'a> {
        match self {
            Error::Recoverable { source, .. } | Error::Unrecoverable { source, .. } => source,
            Error::Multiple(errors) => {
                if let Some(err) = errors.first() {
                    err.source()
                } else {
                    unimplemented!("There should always be at least one error present");
                }
            }
        }
    }

    pub fn is_eof(&self) -> bool {
        match self {
            Self::Recoverable { is_eof, .. } | Self::Unrecoverable { is_eof, .. } => *is_eof,
            Self::Multiple(errors) => {
                let mut is_eof = true;
                for error in errors {
                    if !error.is_eof() {
                        is_eof = false;
                    }
                }
                is_eof
            }
        }
    }
}

impl<'a> From<Error<'a>> for Template<'a> {
    fn from(error: Error<'a>) -> Self {
        let mut items = Vec::with_capacity(1);
        match error {
            Error::Recoverable {
                message,
                source,
                previous_error: _,
                is_eof: _,
            }
            | Error::Unrecoverable {
                message,
                source,
                previous_error: _,
                is_eof: _,
            } => {
                items.push(Item::CompileError {
                    message,
                    error_source: source.clone(),
                    consumed_source: source,
                });
            }
            Error::Multiple(errors) => {
                for error in errors {
                    let Template(additional_items) = error.into();
                    items.extend(additional_items);
                }
            }
        }

        Self(items)
    }
}

/// Estimated length of template output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct EstimatedLength(usize);

impl EstimatedLength {
    pub fn new(estimated_length: usize) -> Self {
        Self(estimated_length)
    }
}

impl Add<EstimatedLength> for EstimatedLength {
    type Output = EstimatedLength;

    fn add(self, rhs: EstimatedLength) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<EstimatedLength> for EstimatedLength {
    fn add_assign(&mut self, rhs: EstimatedLength) {
        self.0 += rhs.0;
    }
}

impl Mul<usize> for EstimatedLength {
    type Output = EstimatedLength;

    fn mul(self, rhs: usize) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl ToTokens for EstimatedLength {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

/// `ToTokens` but with immutable state.
/// Used by expressions and patterns.
trait ToTokensWithState<'a> {
    #[must_use]
    fn to_tokens_with_state(&self, state: &State<'a>) -> BuiltTokens;
}

/// `ToTokens` but with mutable state.
/// Used by statements.
trait ToTokensWithMutState<'a> {
    #[must_use]
    fn to_tokens_with_mut_state<'b: 'a>(&'a self, state: &mut State<'b>) -> BuiltTokens;
}
