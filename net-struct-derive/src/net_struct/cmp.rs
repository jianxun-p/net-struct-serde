use std::str::FromStr;

use super::*;
use field::NetStructFieldType;
use proc_macro2::TokenStream;
use quote::quote;

const ACC_VAR_NAME: &'static str = "is_same";

impl NetStruct {
    /**
     * writes the implements of Eq for the NetStruct
     */
    pub(super) fn comparer(&self) -> Result<TokenStream, DeriveErr> {
        let mut fields_serialize_ts = TokenStream::new();
        let all_vec = self.find_all_vec_fields();
        let var = TokenStream::from_str(ACC_VAR_NAME).unwrap();
        for f in self.fields.iter().filter(|f| !f.is_phantom()) {
            let field_name = TokenStream::from_str(f.name.as_str()).unwrap();
            fields_serialize_ts.extend(match &f.ty {
                NetStructFieldType::Val { ty: _, } => {
                    quote! { #var &= self.#field_name == other.#field_name; }
                },
                NetStructFieldType::Arr { ty: _, capacity: _, } => {
                    quote! { #var &= self.#field_name == other.#field_name; }
                },
                NetStructFieldType::Vec { ty, capacity: _, } => {
                    let Some(v_f) = all_vec.get(&f.name) else {
                        return Err(DeriveErr::Custoum(format!(
                            "Unexpected error when implementing core::cmp::Eq for the vector field \"{}\" of the structure \"{}\"", 
                            &f.name, 
                            self.derive_input.ident.to_string())));
                    };
                    let len_field = TokenStream::from_str(v_f.len_field.name.as_str()).unwrap();
                    let unit = match v_f.len_unit {
                        SizeUnit::BITS => quote!(as usize / (8_usize * core::mem::size_of::<#ty>())),
                        SizeUnit::BYTES => quote!(as usize / core::mem::size_of::<#ty>()),
                        SizeUnit::LENGTH => quote!(as usize),
                    };
                    quote! {
                        #var &= (0..(self.#len_field #unit))
                            .fold(true, |acc, i| acc && self.#field_name[i] == other.#field_name[i]);
                    }
                },
            });
        }
        let struct_name = &self.derive_input.ident;
        Ok(quote! {
            impl core::cmp::PartialEq<Self> for #struct_name {
                fn eq(&self, other: &Self) -> bool {
                    let mut #var = true;
                    #fields_serialize_ts
                    #var
                }
            }
            impl core::cmp::Eq for #struct_name { }
        })
    }
}
