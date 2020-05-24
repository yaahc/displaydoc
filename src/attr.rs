use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    spanned::Spanned, Attribute, Lit::Str, LitStr, Meta, MetaList, MetaNameValue, NestedMeta,
    Result,
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
            Meta::NameValue(MetaNameValue { lit: Str(lit), .. }) => Some(lit),
            Meta::NameValue(_) => unimplemented!("namevalue"),
            Meta::Path(_) => unimplemented!("path"),
            Meta::List(MetaList { nested, .. }) => {
                nested.iter().find_map(|nested_attr| match nested_attr {
                    NestedMeta::Meta(Meta::Path(path)) => {
                        if path.is_ident("transparent") {
                            Some(LitStr::new("{0}", attr.span()))
                        } else {
                            unimplemented!()
                        }
                    }
                    NestedMeta::Lit(Str(lit)) => Some(lit.clone()),
                    _ => unimplemented!(),
                })
            }
        };
        if let Some(l) = lit {
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
