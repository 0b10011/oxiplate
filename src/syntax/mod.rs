mod comment;
mod expression;
mod item;
mod statement;
mod r#static;
mod template;
mod writ;

use nom::error::VerboseError;
use nom::IResult;

pub(self) type Res<T, U> = IResult<T, U, VerboseError<T>>;
pub(self) type Span<'a> = &'a str;

pub(self) use expression::Expression;
pub(self) use item::Item;
pub(self) use r#static::Static;
pub(self) use statement::Statement;
pub(crate) use template::parse;
pub use template::Template;
pub(self) use writ::Writ;
