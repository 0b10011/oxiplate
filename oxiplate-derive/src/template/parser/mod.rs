mod comment;
mod expression;
mod item;
mod statement;
mod r#static;
mod template;
mod writ;

use self::item::Item;
use self::statement::Statement;
use self::r#static::Static;
use self::template::Template;
pub(crate) use self::template::parse;
use self::writ::Writ;
use super::tokenizer::TokenKind;
use crate::Source;
use crate::parser::Error;

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
