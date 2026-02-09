mod comment;
mod expression;
mod item;
mod statement;
mod r#static;
mod template;
mod writ;

use item::Item;
use r#static::Static;
use statement::Statement;
pub(crate) use template::parse;
use writ::Writ;

use crate::parser::Error;
use crate::template::parser::template::Template;
use crate::template::tokenizer::TokenKind;
use crate::Source;

type Res<'a, S> = crate::parser::Res<'a, TokenKind, S>;

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

        let additional_errors: Vec<Error<'a>> = match error {
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

                // Ignore previous errors
                vec![]
            }
            Error::Multiple(errors) => errors,
        };

        for error in additional_errors {
            let Template(additional_items) = error.into();
            items.extend(additional_items);
        }

        Self(items)
    }
}
