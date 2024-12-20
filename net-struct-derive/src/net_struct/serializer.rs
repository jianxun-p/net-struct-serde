use field::NetStructFieldType;
use quote::quote;
use std::str::FromStr;

use super::*;
use crate::err::*;

impl NetStruct {
    /**
     * writes the implements of Serialize for the NetStruct
     */
    pub(super) fn serializer(&self) -> Result<TokenStream, DeriveErr> {
        let mut fields_serialize_ts = TokenStream::new();
        let var = TokenStream::from_str("tup").unwrap();
        let all_vec = self.find_all_vec_fields();
        for f in self.fields.iter() {
            let field_name = TokenStream::from_str(f.name.as_str()).unwrap();
            fields_serialize_ts.extend(match &f.ty {
                NetStructFieldType::Val { ty } => {
                    quote! {#var.serialize_element::<#ty>(&self.#field_name)?;}
                }
                NetStructFieldType::Arr { ty, capacity: _ } => {
                    if let Some(v_f) = all_vec.get(&f.name) {
                        let len_field = TokenStream::from_str(v_f.len_field.name.as_str()).unwrap();
                        let unit = match v_f.len_unit {
                            SizeUnit::BITS => {
                                quote!(as usize / (8_usize * core::mem::size_of::<#ty>()))
                            }
                            SizeUnit::BYTES => quote!(as usize / core::mem::size_of::<#ty>()),
                            SizeUnit::LENGTH => quote!(as usize),
                        };
                        quote! {
                            for i in self.#field_name[0..(self.#len_field #unit)].iter() {
                                #var.serialize_element::<#ty>(i)?;
                            }
                        }
                    } else {
                        quote! {
                            for i in self.#field_name.iter() {
                                #var.serialize_element::<#ty>(i)?;
                            }
                        }
                    }
                }
            });
        }

        let struct_name = &self.derive_input.ident;
        let num_fields = self.fields.len();
        Ok(quote! {
            impl serde::Serialize for #struct_name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where S: serde::Serializer
                {
                    use serde::ser::SerializeTuple;
                    let mut #var = serializer.serialize_tuple(#num_fields)?;
                    #fields_serialize_ts
                    #var.end()
                }
            }
        })
    }
}
