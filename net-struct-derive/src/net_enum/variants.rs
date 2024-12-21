use quote::ToTokens;

#[derive(Debug, Clone)]
pub(super) struct NetEnumVariants {
    pub ident: proc_macro2::Ident,
    pub discriminant: Option<proc_macro2::TokenStream>,
}

impl From<&syn::Variant> for NetEnumVariants {
    fn from(value: &syn::Variant) -> Self {
        Self {
            ident: value.ident.clone(),
            discriminant: value.discriminant.clone().map(|(_, e)| e.to_token_stream()),
        }
    }
}
