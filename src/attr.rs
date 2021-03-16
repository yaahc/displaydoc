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
    let num_doc_attrs = attrs
        .iter()
        .filter(|attr| attr.path.is_ident("doc"))
        .count();
    if num_doc_attrs > 1 {
        panic!("Multi-line comments are not currently supported by displaydoc. Please consider using block doc comments (/** */)");
    }

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

            // Make an attempt and cleaning up multiline doc comments
            let doc_str = lit
                .value()
                .lines()
                .map(|line| line.trim().trim_start_matches('*').trim())
                .collect::<Vec<&str>>()
                .join("\n");

            let lit = LitStr::new(doc_str.trim(), lit.span());

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
