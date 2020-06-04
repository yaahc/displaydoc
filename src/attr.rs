use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    spanned::Spanned, Attribute, Error, Lit::Str, LitStr, Meta, MetaList, MetaNameValue,
    NestedMeta, Result,
};

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
    let attr = attrs
        .iter()
        .find(|attr| attr.path.is_ident("display"))
        .or_else(|| attrs.iter().find(|attr| attr.path.is_ident("doc")));
    if let Some(attr) = attr {
        let meta = attr.parse_meta()?;
        let lit = match meta {
            Meta::NameValue(MetaNameValue { lit: Str(lit), .. }) => Some(Ok(lit)),
            Meta::List(MetaList { nested, .. }) => {
                nested.iter().find_map(|nested_attr| match nested_attr {
                    NestedMeta::Meta(Meta::Path(path)) => {
                        if path.is_ident("transparent") {
                            Some(Ok(LitStr::new("{0}", attr.span())))
                        } else {
                            Some(Err(Error::new_spanned(attr, "attr error")))
                        }
                    }
                    NestedMeta::Lit(Str(lit)) => Some(Ok(lit.clone())),
                    _ => Some(Err(Error::new_spanned(attr, "cant accept the type"))),
                })
            }
            _ => Some(Err(Error::new_spanned(attr, "namevalue or meta"))),
        };
        if let Some(Ok(l)) = lit {
            let mut display = Display {
                fmt: LitStr::new(l.value().trim(), l.span()),
                args: TokenStream::new(),
            };

            display.expand_shorthand();
            return Ok(Some(display));
        };
    }

    Ok(None)
}
