use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, LitStr, Meta, Result};

pub struct Display {
    pub fmt: LitStr,
    pub args: TokenStream,
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

            let lit = LitStr::new(
                &lit.value()
                    .trim()
                    // Deal with rustc including unnecessary parts of the doc comment :(
                    .replace("\n     *", "\n")
                    .trim_start_matches("* "),
                lit.span(),
            );

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
