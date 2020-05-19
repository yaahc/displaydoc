use syn::{DeriveInput, Meta, NestedMeta};

pub struct DisplayDocInput {
    pub with_thiserror: Option<bool>,
}

impl DisplayDocInput {
    pub fn new(input: &DeriveInput) -> DisplayDocInput {
        let mut with_thiserror = None;

        let meta = input.attrs.iter().find_map(|attr| match attr.parse_meta() {
            Ok(m) => {
                if m.path().is_ident("displaydoc") {
                    Some(m)
                } else {
                    None
                }
            }
            Err(e) => panic!("unable to parse attribute: {}", e),
        });

        if let Some(syn::Meta::List(inner)) = meta {
            for item in inner.nested {
                if let NestedMeta::Meta(Meta::NameValue(ref pair)) = item {
                    if pair.path.is_ident("with_thiserror") {
                        if let syn::Lit::Bool(ref s) = pair.lit {
                            with_thiserror = Some(s.value);
                        } else {
                            panic!("with_thiserror arg must be boolean");
                        }
                    } else if let Some(ident) = pair.path.get_ident() {
                        panic!("Attribute {:?} is not supported", ident.to_string())
                    }
                };
            }
        };
        DisplayDocInput { with_thiserror }
    }
}
