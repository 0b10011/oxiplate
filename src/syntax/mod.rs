mod comment;
mod expression;
mod item;
mod statement;
mod r#static;
mod template;
mod writ;

use nom::error::VerboseError;
use nom::IResult;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

use expression::Expression;
use item::Item;
use r#static::Static;
use statement::Statement;
pub(crate) use template::parse;
use writ::Writ;
