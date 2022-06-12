#![feature(proc_macro_diagnostic)]
#![doc(issue_tracker_base_url = "https://github.com/0b10011/Oxiplate/issues/")]
#![doc(test(no_crate_inject))]
#![doc(test(attr(deny(warnings))))]
#![doc = include_str!("../README.md")]

mod syntax;

use nom::Compare;
use nom::InputIter;
use nom::InputLength;
use nom::InputTake;
use nom::Needed;
use nom::Offset;
use nom::Slice;
use nom::UnspecializedInput;
use proc_macro::TokenStream;
use proc_macro2::Literal;
use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use std::ops::Range;
use std::ops::RangeFrom;
use std::ops::RangeTo;
use std::path::PathBuf;
use std::str::CharIndices;
use std::str::Chars;
use std::{env, fs};
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Fields};

#[derive(Debug)]
pub(crate) struct SourceOwned {
    code: String,
    literal: Literal,
    origin: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub(crate) struct Source<'a> {
    original: &'a SourceOwned,
    range: Range<usize>,
}

impl<'a> Source<'a> {
    pub fn as_str(&self) -> &'a str {
        &self.original.code[self.range.clone()]
    }

    pub fn span(&self) -> Span {
        self.original
            .literal
            .subspan(Range {
                start: self.range.start + 1,
                end: self.range.end + 1,
            })
            .unwrap_or_else(proc_macro2::Span::call_site)
    }
}

impl<'a> Slice<RangeFrom<usize>> for Source<'a> {
    fn slice(&self, new_range: RangeFrom<usize>) -> Self {
        Source {
            original: self.original,
            range: Range {
                start: self.range.start + new_range.start,
                end: self.range.end,
            },
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Source<'a> {
    fn slice(&self, new_range: RangeTo<usize>) -> Self {
        Source {
            original: self.original,
            range: Range {
                start: self.range.start,
                end: self.range.start + new_range.end,
            },
        }
    }
}

impl<'a> ToTokens for Source<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        let text = self.as_str();
        let span = self
            .original
            .literal
            .subspan(self.range.clone())
            .expect("No subspan found");
        tokens.append_all(quote::quote_spanned! {span=> write!(f, "{}", #text)?;});
    }
}

impl<'a> PartialEq<Source<'a>> for Source<'a> {
    fn eq(&self, other: &Source) -> bool {
        self.range == other.range
            && self.original.origin == other.original.origin
            && self.original.code == other.original.code
    }
}

impl<'a> Eq for Source<'a> {}

impl<'a> PartialEq<char> for Source<'a> {
    fn eq(&self, char: &char) -> bool {
        self.as_str().len() == 1 && char == &self.as_str().chars().next().unwrap()
    }
}

impl<'a> Compare<&str> for Source<'a> {
    fn compare(&self, string: &str) -> nom::CompareResult {
        self.as_str().compare(string)
    }

    fn compare_no_case(&self, string: &str) -> nom::CompareResult {
        self.as_str().compare_no_case(string)
    }
}

impl<'a> InputIter for Source<'a> {
    type Item = char;
    type Iter = CharIndices<'a>;
    type IterElem = Chars<'a>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.as_str().iter_indices()
    }

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.as_str().iter_elements()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.as_str().position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        self.as_str().slice_index(count)
    }
}

impl<'a> InputTake for Source<'a> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        let end = self.range.start + count;
        if end > self.range.end {
            panic!("End greater than end of string");
        }
        Source {
            original: self.original,
            range: Range {
                start: self.range.start,
                end,
            },
        }
    }

    // return byte index
    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let end = self.range.start + count;
        if end > self.range.end {
            panic!("End greater than end of string");
        }

        (
            Source {
                original: self.original,
                range: Range {
                    start: end,
                    end: self.range.end,
                },
            },
            Source {
                original: self.original,
                range: Range {
                    start: self.range.start,
                    end,
                },
            },
        )
    }
}

impl<'a> InputLength for Source<'a> {
    fn input_len(&self) -> usize {
        self.as_str().input_len()
    }
}

impl<'a> Offset for Source<'a> {
    fn offset(&self, offset: &Self) -> usize {
        self.as_str().offset(offset.as_str())
    }
}

impl<'a> UnspecializedInput for Source<'a> {}

impl<'a> Iterator for Source<'a> {
    type Item = Source<'a>;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        todo!()
    }
}

#[proc_macro_derive(Oxiplate, attributes(oxi_code, oxi_path))]
pub fn oxiplate(input: TokenStream) -> TokenStream {
    match parse(input) {
        Ok(token_stream) => token_stream,
        Err(err) => err.to_compile_error().into(),
    }
}

fn parse(input: TokenStream) -> Result<TokenStream, syn::Error> {
    let input = syn::parse(input).unwrap();
    let DeriveInput {
        attrs, ident, data, ..
    } = &input;

    let mut field_names: Vec<&syn::Ident> = Vec::new();
    match data {
        Data::Struct(ref struct_item) => {
            if let Fields::Named(fields) = &struct_item.fields {
                for field in &fields.named {
                    match &field.ident {
                        Some(name) => field_names.push(name),
                        None => field.span().unwrap().error("Expected a named field").emit(),
                    }
                }
            }
        }
        _ => {
            return Err(syn::Error::new(input.span(), "Expected a struct"));
        }
    };

    let source = get_source(attrs)?;
    let source = Source {
        original: &source,
        range: Range {
            start: 0,
            end: source.code.len(),
        },
    };
    let template = syntax::parse(source, &field_names);

    let expanded = quote! {
        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #template
                Ok(())
            }
        }
    };

    Ok(TokenStream::from(expanded))
}

fn get_source(attrs: &Vec<Attribute>) -> Result<SourceOwned, syn::Error> {
    let invalid_attribute_message = r#"Must provide either an external or internal template:
External: #[oxi_path = "/absolute/path/to/template/within/project.txt.oxip"]
External: #[oxi_path = "./relative/path/to/template/from/current/file.txt.oxip"]
Internal: #[oxi_code = "{{ your_var }}"]"#;
    for attr in attrs {
        if attr.path.is_ident("oxi_code") {
            return match attr.parse_meta() {
                Ok(syn::Meta::NameValue(syn::MetaNameValue {
                    lit: syn::Lit::Str(code),
                    ..
                })) => Ok(SourceOwned {
                    code: code.value(),
                    literal: code.token(),
                    origin: None,
                }),
                Err(err) => Err(err),
                _ => Err(syn::Error::new(attr.span(), invalid_attribute_message)),
            };
        } else if attr.path.is_ident("oxi_path") {
            return match attr.parse_meta().expect("Unable to parse attribute") {
                syn::Meta::NameValue(syn::MetaNameValue {
                    lit: syn::Lit::Str(path),
                    ..
                }) => {
                    let base_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
                        .canonicalize()
                        .expect("Could not canonicalize CARGO_MANIFEST_DIR");
                    let path = PathBuf::from(path.value());
                    let path = if path.starts_with("/") {
                        base_path.join(
                            path.strip_prefix("/")
                                .expect("Could not strip leading slash"),
                        )
                    } else {
                        base_path
                            .join(
                                PathBuf::from(file!())
                                    .parent()
                                    .expect("Could not get parent directory of current file"),
                            )
                            .join(path)
                    };
                    let path = path.canonicalize().expect("Could not canonicalize");

                    if !path.starts_with(&base_path) {
                        panic!("Path {:?} must start with {:?}", path, base_path);
                    }

                    let code = fs::read_to_string(&path).expect("Could not read file");
                    let literal = Literal::string(&code);
                    Ok(SourceOwned {
                        code,
                        literal,
                        origin: Some(path),
                    })
                }
                _ => Err(syn::Error::new(attr.span(), invalid_attribute_message)),
            };
        }
    }

    unimplemented!();
}
