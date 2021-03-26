use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Attribute, LitStr, Meta, Result};

pub(crate) struct Display {
    pub(crate) fmt: LitStr,
    pub(crate) args: TokenStream,
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

pub(crate) struct AttrsHelper {
    ignore_extra_doc_attributes: bool,
}

impl AttrsHelper {
    pub(crate) fn new(attrs: &[Attribute]) -> Self {
        let ignore_extra_doc_attributes = attrs
            .iter()
            .any(|attr| attr.path.is_ident("ignore_extra_doc_attributes"));

        Self {
            ignore_extra_doc_attributes,
        }
    }

    pub(crate) fn display(&self, attrs: &[Attribute]) -> Result<Option<Display>> {
        let num_doc_attrs = attrs
            .iter()
            .filter(|attr| attr.path.is_ident("doc"))
            .count();

        if !self.ignore_extra_doc_attributes && num_doc_attrs > 1 {
            panic!("Multi-line comments are disabled by default by displaydoc. Please consider using block doc comments (/** */) or adding the #[ignore_extra_doc_attributes] attribute to your type next to the derive.");
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
}
