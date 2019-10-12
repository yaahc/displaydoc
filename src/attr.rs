use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote, ToTokens};
use std::iter::once;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Ident, Index, LitInt, LitStr, Meta, Result, Token};

pub struct Display {
    pub fmt: LitStr,
    pub args: TokenStream,
}

impl Parse for Display {
    fn parse(input: ParseStream) -> Result<Self> {
        dbg!("hi");
        let fmt: LitStr = input.parse()?;

        let mut args = TokenStream::new();
        let mut last_is_comma = false;
        while !input.is_empty() {
            if last_is_comma && input.peek(Token![.]) {
                if input.peek2(Ident) {
                    input.parse::<Token![.]>()?;
                    last_is_comma = false;
                    continue;
                }
                if input.peek2(LitInt) {
                    input.parse::<Token![.]>()?;
                    let int: Index = input.parse()?;
                    let ident = format_ident!("_{}", int.index, span = int.span);
                    args.extend(once(TokenTree::Ident(ident)));
                    last_is_comma = false;
                    continue;
                }
            }
            last_is_comma = input.peek(Token![,]);
            let token: TokenTree = input.parse()?;
            args.extend(once(token));
        }

        let mut display = Display { fmt, args };
        display.expand_shorthand();
        Ok(display)
    }
}

impl ToTokens for Display {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let fmt = &self.fmt;
        let args = &self.args;
        tokens.extend(quote! {
            write!(formatter, #fmt #args)
        });
    }
}

pub fn display(attrs: &[Attribute]) -> Result<Option<Display>> {
    for attr in attrs {
        if attr.path.is_ident("doc") {
            let meta = attr.parse_meta()?;
            let lit = match meta {
                Meta::NameValue(syn::MetaNameValue {
                    lit: syn::Lit::Str(lit),
                    ..
                }) => lit,
                _ => unimplemented!(),
            };

            let fmt = lit.value();
            let fmt = fmt.trim();
            // .lines()
            // .next()
            // .expect("expect: input doc attribute must have at least 1 non empty line");

            let lit = LitStr::new(fmt, lit.span());

            let mut display = Display {
                fmt: lit,
                args: TokenStream::new(),
            };

            display.expand_shorthand();
            return Ok(Some(display));
        }
    }

    Ok(None)
}
