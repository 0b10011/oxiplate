use super::{super::Source, item::parse_tag, r#static::parse_static, Item, Res, Static};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::{eof, opt};
use nom::multi::many0;
use nom::sequence::tuple;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Template<'a>(pub Vec<Item<'a>>);

impl ToTokens for Template<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for item in &self.0 {
            tokens.append_all(quote! { #item });
        }
    }
}

pub(crate) fn parse<'a>(source: Source<'a>) -> Template<'a> {
    match try_parse(source.clone()) {
        Ok((_, template)) => template,
        Err(nom::Err::Error(nom::error::VerboseError { errors }))
        | Err(nom::Err::Failure(nom::error::VerboseError { errors })) => {
            let origin = match &source.original.origin {
                Some(origin) => format!("Syntax error in {}", origin.display()),
                None => "Syntax error in inline template".into(),
            };
            let mut new_errors = vec![];
            for (input, error) in errors {
                new_errors.push((input.as_str(), error));
            }
            Template(vec![Item::CompileError(format!(
                "{}:\n{}",
                origin,
                nom::error::convert_error(
                    source.as_str(),
                    nom::error::VerboseError { errors: new_errors }
                )
            ))])
        }
        Err(nom::Err::Incomplete(_)) => {
            unreachable!("This should only happen in nom streams which aren't used by Oxiplate.")
        }
    }
}

fn try_parse(
    source: Source,
) -> Res<Source, Template> {
    let (input, items_vec) = many0(parse_item)(source)?;

    // Return error if there's any input remaining.
    // Successful value is `("", "")`, so no need to capture.
    let (input, _) = eof(input)?;

    let mut items = Vec::new();
    for mut item_vec in items_vec {
        items.append(&mut item_vec);
    }

    Ok((input, Template(items)))
}

pub(crate) fn parse_item(input: Source) -> Res<Source, Vec<Item>> {
    alt((parse_tag, parse_static, adjusted_whitespace))(input)
}

pub(crate) fn adjusted_whitespace(input: Source) -> Res<Source, Vec<Item>> {
    let (input, (leading_whitespace, tag, trailing_whitespace)) = tuple((
        opt(whitespace),
        alt((tag("{_}"), tag("{-}"))),
        opt(whitespace),
    ))(input)?;

    let whitespace = match tag.as_str() {
        "{_}" => {
            if let Some(leading_whitespace) = leading_whitespace {
                vec![Static(" ", leading_whitespace).into()]
            } else if let Some(trailing_whitespace) = trailing_whitespace {
                vec![Static(" ", trailing_whitespace).into()]
            } else {
                vec![]
            }
        }
        "{-}" => vec![],
        _ => unreachable!("Only whitespace control tags should be matched"),
    };

    Ok((input, whitespace))
}

// https://doc.rust-lang.org/reference/whitespace.html
pub fn is_whitespace(char: char) -> bool {
    matches!(
        char,
        '\u{0009}' // (horizontal tab, '\t')
        | '\u{000A}' // (line feed, '\n')
        | '\u{000B}' // (vertical tab)
        | '\u{000C}' // (form feed)
        | '\u{000D}' // (carriage return, '\r')
        | '\u{0020}' // (space, ' ')
        | '\u{0085}' // (next line)
        | '\u{200E}' // (left-to-right mark)
        | '\u{200F}' // (right-to-left mark)
        | '\u{2028}' // (line separator)
        | '\u{2029}' // (paragraph separator)
    )
}

pub(crate) fn whitespace(input: Source) -> Res<Source, Source> {
    take_while1(is_whitespace)(input)
}

#[test]
fn test_empty() {
    use std::ops::Range;

    let code = "".to_owned();
    let literal = proc_macro2::Literal::string(&code);
    let original_source = crate::SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 0 },
            },
        ),
        Template(vec![])
    );
}

#[test]
fn test_word() {
    use std::ops::Range;

    let code = "Test".to_owned();
    let literal = proc_macro2::Literal::string(&code);
    let original_source = crate::SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 4 }
            },
        ),
        Template(vec![Item::Static(Static(
            "Test",
            Source {
                original: &original_source,
                range: Range { start: 0, end: 4 }
            }
        ))])
    );
}

#[test]
fn test_phrase() {
    use std::ops::Range;

    let code = "Some text.".to_owned();
    let literal = proc_macro2::Literal::string(&code);
    let original_source = crate::SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 10 }
            },
        ),
        Template(vec![Item::Static(Static(
            "Some text.",
            Source {
                original: &original_source,
                range: Range { start: 0, end: 10 }
            }
        ))])
    );
}

#[test]
fn test_stray_brace() {
    use std::ops::Range;

    let code = "Some {text}.".to_owned();
    let literal = proc_macro2::Literal::string(&code);
    let original_source = crate::SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 12 }
            },
        ),
        Template(vec![Item::Static(Static(
            "Some {text}.",
            Source {
                original: &original_source,
                range: Range { start: 0, end: 12 }
            }
        ))])
    );
}

#[test]
fn test_writ() {
    use std::ops::Range;

    let code = "{{ greeting }}".to_owned();
    let literal = proc_macro2::Literal::string(&code);
    let original_source = crate::SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 14 }
            },
        ),
        Template(vec![Item::Writ(super::Writ(
            super::Expression::Identifier(super::expression::IdentifierOrFunction::Identifier(
                super::expression::Identifier(
                    "greeting",
                    Source {
                        original: &original_source,
                        range: Range { start: 3, end: 11 }
                    }
                )
            ))
        )),])
    );
}

#[test]
fn test_trimmed_whitespace() {
    use std::ops::Range;

    let code = "Hello \t\n {-} \t\n world!".to_owned();
    let literal = proc_macro2::Literal::string(&code);
    let original_source = crate::SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 22 },
            },
        ),
        Template(vec![
            Item::Static(Static(
                "Hello",
                Source {
                    original: &original_source,
                    range: Range { start: 0, end: 5 },
                }
            )),
            Item::Static(Static(
                "world!",
                Source {
                    original: &original_source,
                    range: Range { start: 16, end: 22 },
                }
            )),
        ])
    );
}

#[test]
fn test_trimmed_leading_whitespace() {
    use std::ops::Range;

    let code = "Hello \t\n {{- greeting }}".to_owned();
    let literal = proc_macro2::Literal::string(&code);
    let original_source = crate::SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 24 },
            },
        ),
        Template(vec![
            Item::Static(Static(
                "Hello",
                Source {
                    original: &original_source,
                    range: Range { start: 0, end: 5 },
                }
            )),
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "greeting",
                    Source {
                        original: &original_source,
                        range: Range { start: 13, end: 21 },
                    }
                ))
            ))),
        ])
    );
}

#[test]
fn test_trimmed_trailing_whitespace() {
    use std::ops::Range;

    let code = "{{ greeting -}} \t\n !".to_owned();
    let literal = proc_macro2::Literal::string(&code);
    let original_source = crate::SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 20 },
            },
        ),
        Template(vec![
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "greeting",
                    Source {
                        original: &original_source,
                        range: Range { start: 3, end: 11 },
                    }
                ))
            ))),
            Item::Static(Static(
                "!",
                Source {
                    original: &original_source,
                    range: Range { start: 19, end: 20 },
                }
            )),
        ])
    );
}

#[test]
fn test_collapsed_whitespace() {
    use std::ops::Range;

    let code = "Hello \t\n {_} \t\n world!".to_owned();
    let literal = proc_macro2::Literal::string(&code);
    let original_source = crate::SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 22 },
            },
        ),
        Template(vec![
            Item::Static(Static(
                "Hello",
                Source {
                    original: &original_source,
                    range: Range { start: 0, end: 5 },
                }
            )),
            Item::Static(Static(
                " ",
                Source {
                    original: &original_source,
                    range: Range { start: 5, end: 9 },
                }
            )),
            Item::Static(Static(
                "world!",
                Source {
                    original: &original_source,
                    range: Range { start: 16, end: 22 },
                }
            )),
        ])
    );
}

#[test]
fn test_collapsed_leading_whitespace() {
    use std::ops::Range;

    let code = "Hello \t\n {{_ greeting }}".to_owned();
    let literal = proc_macro2::Literal::string(&code);
    let original_source = crate::SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 24 },
            },
        ),
        Template(vec![
            Item::Static(Static(
                "Hello",
                Source {
                    original: &original_source,
                    range: Range { start: 0, end: 5 },
                }
            )),
            Item::Static(Static(
                " ",
                Source {
                    original: &original_source,
                    range: Range { start: 5, end: 9 },
                }
            )),
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "greeting",
                    Source {
                        original: &original_source,
                        range: Range { start: 13, end: 21 },
                    }
                ))
            ))),
        ])
    );
}

#[test]
fn test_collapsed_trailing_whitespace_writ() {
    use std::ops::Range;

    let code = "{{ greeting _}} \t\n world!".to_owned();
    let literal = proc_macro2::Literal::string(&code);
    let original_source = crate::SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 25 },
            },
        ),
        Template(vec![
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "greeting",
                    Source {
                        original: &original_source,
                        range: Range { start: 3, end: 11 },
                    }
                ))
            ))),
            Item::Static(Static(
                " ",
                Source {
                    original: &original_source,
                    range: Range { start: 15, end: 19 },
                }
            )),
            Item::Static(Static(
                "world!",
                Source {
                    original: &original_source,
                    range: Range { start: 19, end: 25 },
                }
            )),
        ])
    );
}

#[test]
fn test_collapsed_trailing_whitespace_comment() {
    use std::ops::Range;

    let code = "Hello {#- Some comment _#} \t\n world!".to_owned();
    let literal = proc_macro2::Literal::string(&code);
    let original_source = crate::SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 36 },
            },
        ),
        Template(vec![
            Item::Static(Static(
                "Hello",
                Source {
                    original: &original_source,
                    range: Range { start: 0, end: 5 },
                }
            )),
            Item::Comment,
            Item::Static(Static(
                " ",
                Source {
                    original: &original_source,
                    range: Range { start: 26, end: 30 },
                }
            )),
            Item::Static(Static(
                "world!",
                Source {
                    original: &original_source,
                    range: Range { start: 30, end: 36 },
                }
            )),
        ])
    );
}

#[test]
fn test_collapsed_whitespace_comment_no_whitespace() {
    use std::ops::Range;

    let code = "Hello{#_ Some comment _#}world!".to_owned();
    let literal = proc_macro2::Literal::string(&code);
    let original_source = crate::SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 31 },
            },
        ),
        Template(vec![
            Item::Static(Static(
                "Hello",
                Source {
                    original: &original_source,
                    range: Range { start: 0, end: 5 },
                }
            )),
            Item::Comment,
            Item::Static(Static(
                "world!",
                Source {
                    original: &original_source,
                    range: Range { start: 25, end: 31 },
                }
            )),
        ])
    );
}

#[test]
fn test_collapsed_whitespace_writ_no_whitespace() {
    use crate::SourceOwned;
    use proc_macro2::Literal;
    use std::ops::Range;

    let code: String = "Hello{{_ variable _}}world!".into();
    let literal = Literal::string(&code);
    let original_source = SourceOwned {
        code,
        literal,
        origin: None,
    };

    assert_eq!(
        parse(
            Source {
                original: &original_source,
                range: Range { start: 0, end: 27 },
            },
        ),
        Template(vec![
            Item::Static(Static(
                "Hello",
                Source {
                    original: &original_source,
                    range: Range { start: 0, end: 5 },
                }
            )),
            Item::Writ(super::Writ(super::Expression::Identifier(
                super::expression::IdentifierOrFunction::Identifier(super::expression::Identifier(
                    "variable",
                    Source {
                        original: &original_source,
                        range: Range { start: 9, end: 17 },
                    }
                ))
            ))),
            Item::Static(Static(
                "world!",
                Source {
                    original: &original_source,
                    range: Range { start: 21, end: 27 },
                }
            )),
        ])
    );
}
