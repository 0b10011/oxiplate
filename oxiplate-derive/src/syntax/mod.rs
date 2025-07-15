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
use r#static::Static;
use statement::Statement;
pub(crate) use template::parse;
use writ::Writ;
