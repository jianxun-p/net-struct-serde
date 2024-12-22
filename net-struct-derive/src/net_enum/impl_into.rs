use proc_macro2::TokenStream;
use quote::quote;

use crate::err::DeriveErr;

use super::NetEnum;

impl NetEnum {
    pub(super) fn impl_into(&self) -> Result<TokenStream, DeriveErr> {
        let ty = &self.attrs.repr;
        let enum_name = &self.derive_input.ident;
        Ok(quote! {
            impl Into<#ty> for #enum_name {
                #[inline]
                fn into(self) -> #ty {
                    self as #ty
                }
            }
            impl #enum_name {
                #[inline]
                pub const fn const_into(self) -> #ty {
                    self as #ty
                }
            }
        })
    }
}
