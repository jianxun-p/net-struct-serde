use std::str::FromStr;

use field::NetStructFieldType;
use quote::quote;
use super::*;
use proc_macro2::TokenStream;

const ACC_VAR_NAME: &'static str = "is_same";


impl NetStruct {

    /**
     * writes the implements of Eq for the NetStruct
     */
    pub(super) fn comparer(&self) -> Result<TokenStream, DeriveErr> {
        let mut fields_serialize_ts = TokenStream::new();
        let all_vec = self.find_all_vec_fields();
        let var = TokenStream::from_str(ACC_VAR_NAME).unwrap();
        for f in self.fields.iter() {
            let field_name = TokenStream::from_str(f.name.as_str()).unwrap();
            fields_serialize_ts.extend(match &f.ty {
                NetStructFieldType::Val { ty: _, } => {
                    quote! { #var &= self.#field_name == other.#field_name; }
                },
                NetStructFieldType::Arr { ty, capacity: _, } => {
                    if let Some(v_f) = all_vec.get(&f.name) {
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
                    } else {
                        quote! { #var &= self.#field_name == other.#field_name; }
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
