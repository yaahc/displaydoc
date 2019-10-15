use crate::attr;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Error, Fields, Result};

pub fn derive(input: &DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Struct(data) => impl_struct(input, data),
        Data::Enum(data) => impl_enum(input, data),
        Data::Union(_) => Err(Error::new_spanned(input, "Unions are not supported")),
    }
}

fn impl_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream> {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let display = attr::display(&input.attrs)?.map(|display| {
        let pat = match &data.fields {
            Fields::Named(fields) => {
                let var = fields.named.iter().map(|field| &field.ident);
                quote!(Self { #(#var),* })
            }
            Fields::Unnamed(fields) => {
                let var = (0..fields.unnamed.len()).map(|i| format_ident!("_{}", i));
                quote!(Self(#(#var),*))
            }
            Fields::Unit => quote!(_),
        };
        quote! {
            impl #impl_generics core::fmt::Display for #ty #ty_generics #where_clause {
                fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    #[allow(unused_variables)]
                    let #pat = self;
                    #display
                }
            }
        }
    });

    Ok(quote! {
        #display
    })
}

fn impl_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream> {
    let ty = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let displays = data
        .variants
        .iter()
        .map(|variant| attr::display(&variant.attrs))
        .collect::<Result<Vec<_>>>()?;

    let display = if displays.iter().any(Option::is_some) {
        let arms = data
            .variants
            .iter()
            .zip(displays)
            .map(|(variant, display)| {
                let display =
                    display.ok_or_else(|| Error::new_spanned(variant, "missing doc comment"))?;
                let ident = &variant.ident;
                Ok(match &variant.fields {
                    Fields::Named(fields) => {
                        let var = fields.named.iter().map(|field| &field.ident);
                        quote!(#ty::#ident { #(#var),* } => #display)
                    }
                    Fields::Unnamed(fields) => {
                        let var = (0..fields.unnamed.len()).map(|i| format_ident!("_{}", i));
                        quote!(#ty::#ident(#(#var),*) => #display)
                    }
                    Fields::Unit => quote!(#ty::#ident => #display),
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Some(quote! {
            impl #impl_generics core::fmt::Display for #ty #ty_generics #where_clause {
                fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    #[allow(unused_variables)]
                    match self {
                        #(#arms,)*
                    }
                }
            }
        })
    } else {
        return Err(Error::new_spanned(input, "Missing doc comments"));
    };

    Ok(quote! {
        #display
    })
}
