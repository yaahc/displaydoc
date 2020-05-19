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
            Meta::NameValue(MetaNameValue { lit: Str(lit), .. }) => lit,
            Meta::NameValue(_) => unimplemented!("namevalue"),
            Meta::Path(_) => unimplemented!("path"),
            Meta::List(MetaList { nested, .. }) => match nested.first() {
                Some(NestedMeta::Meta(Meta::Path(path))) => {
                    if path.segments.first().unwrap().ident == "transparent" {
                        LitStr::new("{0}", attr.span())
                    } else {
                        unimplemented!()
                    }
                }
                Some(NestedMeta::Lit(Str(lit))) => lit.clone(),
                _ => unimplemented!(),
            },
        };

        let lit = LitStr::new(lit.value().trim(), lit.span());

        let mut display = Display {
            fmt: lit,
            args: TokenStream::new(),
        };

        display.expand_shorthand();
        return Ok(Some(display));
    }

    Ok(None)
}
