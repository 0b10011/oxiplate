mod comment;
mod expression;
mod item;
mod parser;
mod statement;
mod r#static;
mod template;
mod writ;

use item::Item;
use statement::Statement;
use r#static::Static;
pub(crate) use template::parse;
use writ::Writ;

use crate::syntax::template::Template;
use crate::{Source, TokenSlice};

type Res<'a, T> = Result<(TokenSlice<'a>, T), Error<'a>>;

#[derive(Debug)]
enum Error<'a> {
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

#[derive(Clone, Debug)]
pub struct UnexpectedTokenError<'a> {
    message: &'a str,
    source: Source<'a>,
    is_eof: bool,
}

impl<'a> UnexpectedTokenError<'a> {
    pub fn new(message: &'a str, source: Source<'a>) -> Self {
        Self {
            message,
            source,
            is_eof: false,
        }
    }

    pub fn eof(source: Source<'a>) -> Self {
        Self {
            message: "End of file encountered",
            source,
            is_eof: true,
        }
    }

    pub fn source(&self) -> &Source<'a> {
        &self.source
    }

    pub fn is_eof(&self) -> bool {
        self.is_eof
    }
}

impl<'a> From<UnexpectedTokenError<'a>> for Error<'a> {
    fn from(value: UnexpectedTokenError<'a>) -> Self {
        let UnexpectedTokenError {
            message,
            source,
            is_eof,
        } = value;

        if is_eof {
            Self::Recoverable {
                message: message.to_string(),
                source,
                previous_error: None,
                is_eof,
            }
        } else {
            Self::Unrecoverable {
                message: message.to_string(),
                source,
                previous_error: None,
                is_eof,
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
