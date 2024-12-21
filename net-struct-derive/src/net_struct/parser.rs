use std::str::FromStr;

use crate::err::DeriveErr;

use super::*;
use field::NetStructFieldType;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{HashMap, HashSet, VecDeque};

struct DeserializeFieldIter {
    net_struct_attrs: NetStructAttr,
    unread_fields: VecDeque<Rc<NetStructField>>,
    read_fields: HashSet<String>,
    vec_fields: HashMap<String, VecField>,
    direction: bool,
}

impl DeserializeFieldIter {
    fn is_deserializable(&self, field: &Rc<NetStructField>) -> bool {
        let Some(v_f) = self.vec_fields.get(&field.name) else {
            return true;
        };
        let already_read_len_field = self.read_fields.contains(&v_f.len_field.name);
        let already_read_struct_len = match self.net_struct_attrs.struct_len {
            Some((ref f, _)) => self.read_fields.contains(&f.name),
            None => true,
        };
        let is_sole_vec = self.unread_fields.len() == 1 && field.is_vec();
        already_read_len_field && already_read_struct_len || is_sole_vec || field.is_phantom()
    }

    pub fn new(net_struct: &NetStruct) -> Self {
        Self {
            net_struct_attrs: net_struct.attrs.clone(),
            unread_fields: VecDeque::from_iter(net_struct.fields.iter().map(|f| f.clone())),
            read_fields: HashSet::new(),
            vec_fields: net_struct.find_all_vec_fields(),
            direction: true,
        }
    }
}

impl std::iter::Iterator for DeserializeFieldIter {
    type Item = Result<(Rc<NetStructField>, bool, bool), DeriveErr>;

    /**
     * Take one field from the VecDeque and returns it with a bool that indicates the current direction
     * and another bool that indicates if the direction has changed
     */
    fn next(&mut self) -> Option<Self::Item> {
        let (front, back) = (
            self.unread_fields.front()?.clone(),
            self.unread_fields.back()?.clone(),
        );
        if !self.is_deserializable(&front) && !self.is_deserializable(&back) {
            return Some(Err(DeriveErr::AmbigiousDeserialize(format!(
                "Unable to deserialize the structure unambigiously from either direction. \n\t\tfront: \"{}\", back: \"{}\"", 
                front.name, back.name
            ))));
        }
        let (field, other_field) = match self.direction {
            true => (front, back),
            false => (back, front),
        };
        let direction_changed = match self.is_deserializable(&field) {
            true => {
                self.read_fields.insert(field.name.clone());
                false
            }
            false => {
                self.read_fields.insert(other_field.name.clone());
                self.direction = !self.direction;
                true
            }
        };
        match self.direction {
            true => Some(Ok((
                self.unread_fields.pop_front()?,
                self.direction,
                direction_changed,
            ))),
            false => Some(Ok((
                self.unread_fields.pop_back()?,
                self.direction,
                direction_changed,
            ))),
        }
    }
}

impl NetStruct {
    const UNINIT_STRUCT_VAR: &'static str = "s";

    fn truncate_size(&self) -> TokenStream {
        if let Some((ref f, unit)) = self.attrs.struct_len {
            let field_name = TokenStream::from_str(f.name.as_str()).unwrap();
            match unit {
                SizeUnit::BITS => quote!(.truncate(#field_name as usize / 8)?),
                SizeUnit::BYTES => quote!(.truncate(#field_name as usize)?),
                SizeUnit::LENGTH => unreachable!(),
            }
        } else {
            TokenStream::new()
        }
    }

    fn deserialize_vec(
        &self,
        field: &Rc<NetStructField>,
        dir: bool,
        _direction_changed: bool,
        vec_fields: &HashMap<String, VecField>,
        ty: &TokenStream,
        _capacity: &String,
    ) -> Result<TokenStream, DeriveErr> {
        let var = TokenStream::from_str(Self::UNINIT_STRUCT_VAR).unwrap();
        let field_name_str = field.name.as_str();
        let field_name = TokenStream::from_str(field_name_str).unwrap();
        let Some(vec_field) = vec_fields.get(&field.name) else {
            return Err(DeriveErr::Message(format!(
                "Unexpected error when implementing Deserialize for the vector field \"{}\" of the structure \"{}\"", 
                &field.name,
                self.derive_input.ident.to_string())));
        };
        let len = TokenStream::from_str(vec_field.len_field.name.as_str()).unwrap();

        match vec_field.len_field.is_phantom() {
            true => {
                let len_adj = match vec_field.len_unit {
                    SizeUnit::BITS => quote!( |l| l / (8_usize * core::mem::size_of::<#ty>())),
                    SizeUnit::BYTES => quote!( |l| l / core::mem::size_of::<#ty>()),
                    SizeUnit::LENGTH => quote!(|l| l),
                };
                Ok(match dir {
                    true => quote! {
                        .deserialize_seq_until_end::<#ty, &mut [#ty]>(&mut (*#var.as_mut_ptr()).#field_name, &mut (*#var.as_mut_ptr()).#len, #len_adj)?
                    },
                    false => quote! {
                        .reverse()?
                        .deserialize_seq_until_end::<#ty, &mut [#ty]>(&mut (*#var.as_mut_ptr()).#field_name, &mut (*#var.as_mut_ptr()).#len, #len_adj)?
                    },
                })
            }
            false => {
                let unit = match vec_field.len_unit {
                    SizeUnit::BITS => quote!(as usize / (8_usize * core::mem::size_of::<#ty>())),
                    SizeUnit::BYTES => quote!(as usize / core::mem::size_of::<#ty>()),
                    SizeUnit::LENGTH => quote!(as usize),
                };
                Ok(quote! {
                    .deserialize_seq::<#ty, &mut [#ty]>(&mut (*#var.as_mut_ptr()).#field_name, #var.assume_init_ref().#len #unit)?
                })
            }
        }
    }

    fn deserialize_one_field(
        &self,
        field: Rc<NetStructField>,
        dir: bool,
        direction_changed: bool,
        vec_fields: HashMap<String, VecField>,
    ) -> Result<TokenStream, DeriveErr> {
        let var = TokenStream::from_str(Self::UNINIT_STRUCT_VAR).unwrap();
        let field_name_str = field.name.as_str();
        let field_name = TokenStream::from_str(field_name_str).unwrap();
        let mut ts = match direction_changed {
            true => quote!(.reverse()?),
            false => quote!(),
        };
        if field.is_phantom() {
            return Ok(ts);
        }
        ts.extend(match &field.ty {
            NetStructFieldType::Val { ty } => quote!{
                .deserialize_field::<#ty>(&mut (*#var.as_mut_ptr()).#field_name, #field_name_str)?
            },
            NetStructFieldType::Arr { ty, capacity } => {
                let capacity_ts = TokenStream::from_str(capacity.as_str()).unwrap();
                quote! {
                    .deserialize_seq::<#ty, &mut [#ty]>(&mut (*#var.as_mut_ptr()).#field_name, #capacity_ts as usize)?
                }
            },
            NetStructFieldType::Vec { ty, capacity } => {
                self.deserialize_vec(&field, dir, direction_changed, &vec_fields, ty, capacity)?
            },
        });
        Ok(ts)
    }

    fn deserialize_fields(&self) -> Result<TokenStream, DeriveErr> {
        let field_iter = DeserializeFieldIter::new(self);
        let mut ts = TokenStream::new();
        for field in field_iter {
            let (f, dir, dir_changed) = field?;
            ts.extend(self.deserialize_one_field(
                f,
                dir,
                dir_changed,
                self.find_all_vec_fields(),
            )?);
        }
        Ok(ts)
    }

    /**
     * writes the implements of Deserialize for the NetStruct
     */
    pub(super) fn parser(&self) -> Result<TokenStream, DeriveErr> {
        let struct_name = &self.derive_input.ident;
        let var = TokenStream::from_str(Self::UNINIT_STRUCT_VAR).unwrap();
        let trunc = self.truncate_size();
        let fields = self.deserialize_fields()?;
        Ok(quote! {
            impl net_struct_serde::traits::Deserialize for #struct_name {
                fn deserialize<D>(deserializer: D) -> Result<Self, net_struct_serde::SerdeErr>
                    where D: net_struct_serde::traits::Deserializer
                {
                    let mut #var = core::mem::MaybeUninit::<#struct_name>::uninit();
                    unsafe {
                        deserializer
                            #trunc
                            #fields;
                        Ok(s.assume_init())
                    }
                }
            }
        })
    }
}
