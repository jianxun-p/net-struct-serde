use std::str::FromStr;

use proc_macro2::TokenStream;

use crate::err::DeriveErr;
use quote::quote;

use super::NetEnum;

impl NetEnum {
    pub(super) fn impl_deserialize(&self) -> Result<TokenStream, DeriveErr> {
        let enum_name = &self.derive_input.ident;
        let ty = &self.attrs.repr;
        let var = TokenStream::from_str("discriminant_val").unwrap();
        Ok(quote! {
            impl net_struct_serde::traits::Deserialize for #enum_name {
                fn deserialize<D>(deserializer: D) -> Result<Self, net_struct_serde::SerdeErr>
                    where D: net_struct_serde::traits::Deserializer
                {
                    use net_struct_serde::traits::Deserialize;
                    let #var = #ty::deserialize(deserializer)?;
                    Self::try_from(#var)
                }
            }
        })
    }
}
