use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::err::DeriveErr;

use super::{NetEnum, NetEnumVariants};
use std::rc::Rc;

impl NetEnum {
    fn var_impl_tryfrom(&self, var: &Rc<NetEnumVariants>) -> Result<TokenStream, DeriveErr> {
        match var.discriminant {
            Some(ref v) => {
                let i = &var.ident;
                let ts = v.to_token_stream();
                Ok(quote! { #ts => Ok(Self::#i), })
            }
            None => Err(DeriveErr::MissingDiscriminant(format!(
                "Expected discriminant for the variant \"{}\" of \"{}\"",
                var.ident.to_string(),
                self.derive_input.ident.to_string(),
            ))),
        }
    }

    pub(super) fn impl_tryfrom(&self) -> Result<TokenStream, DeriveErr> {
        let ty = &self.attrs.repr;
        let enum_name = &self.derive_input.ident;
        let variants = self.variants.iter().fold(TokenStream::new(), |mut acc, i| {
            acc.extend(self.var_impl_tryfrom(i));
            acc
        });
        Ok(quote! {
            impl TryFrom<#ty> for #enum_name {
                type Error = net_struct_serde::SerdeErr;
                fn try_from(value: #ty) -> Result<Self, Self::Error> {
                    match value {
                        #variants
                        _ => Err(net_struct_serde::SerdeErr::ParseFailed)
                    }
                }
            }
        })
    }
}
