use nom::bytes::complete::tag;
use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote_spanned};

use super::Res;
use crate::Source;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Keyword<'a>(pub Source<'a>);

impl ToTokens for Keyword<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let span = self.0.span();
        let keyword = syn::Ident::new(self.0.as_str(), span);
        tokens.append_all(quote_spanned! {span=> #keyword });
    }

    fn to_token_stream(&self) -> TokenStream {
        let mut tokens = TokenStream::new();
        self.to_tokens(&mut tokens);
        tokens
    }

    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.to_token_stream()
    }
}

pub(crate) fn keyword<'a>(
    keyword: &'static str,
) -> impl Fn(Source<'a>) -> Res<Source<'a>, Keyword<'a>> + 'a {
    move |input: Source<'a>| {
        let (input, keyword) = tag(keyword)(input)?;
        Ok((input, Keyword(keyword)))
    }
}
