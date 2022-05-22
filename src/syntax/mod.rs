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

pub use comment::Comment;
pub use expression::Expression;
pub use item::Item;
pub use r#static::Static;
pub use statement::Statement;
pub use template::{parse, Template};
pub use writ::Writ;
