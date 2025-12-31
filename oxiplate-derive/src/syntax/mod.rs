mod comment;
mod expression;
mod item;
mod statement;
mod r#static;
mod template;
mod writ;

use nom::IResult;
use nom_language::error::VerboseError;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

use item::Item;
use statement::Statement;
use r#static::Static;
pub(crate) use template::parse;
use writ::Writ;
