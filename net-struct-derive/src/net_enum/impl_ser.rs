use proc_macro2::TokenStream;

use crate::err::DeriveErr;
use quote::quote;

use super::NetEnum;


impl NetEnum {
    pub(super) fn impl_serialize(&self) -> Result<TokenStream, DeriveErr> {
        let enum_name = &self.derive_input.ident;
        let ty = &self.attrs.repr;
        Ok(quote! {
            impl serde::Serialize for #enum_name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where S: serde::Serializer
                {
                    Into::<#ty>::into(self.clone()).serialize(serializer)
                }
            }
        })
    }
}

