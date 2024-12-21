mod cmp;
mod field;
mod parser;
mod serializer;
use crate::{err::DeriveErr, helper::*};
use field::{FieldAttr, NetStructField, SizeUnit, VecField};
use proc_macro2::{Delimiter, TokenStream};
use quote::quote;
use std::{collections::HashMap, rc::Rc};
use syn::{Data, DeriveInput};

const ATTR_PATH: &'static str = "net_struct";
const STRUCT_SIZE_PATH: &'static str = "struct_len";

#[derive(Clone)]
pub(super) struct NetStruct {
    derive_input: DeriveInput,
    fields: Vec<Rc<NetStructField>>,
    attrs: NetStructAttr,
}

#[derive(Clone)]
struct NetStructAttr {
    struct_len: Option<(Rc<NetStructField>, SizeUnit)>,
}

impl std::cmp::PartialEq for NetStruct {
    fn eq(&self, other: &Self) -> bool {
        self.derive_input.ident.to_string() == other.derive_input.ident.to_string()
    }
}
impl std::cmp::Eq for NetStruct {}



impl NetStruct {

    pub fn derive_input_to_token_stream(di: DeriveInput) -> Result<TokenStream, DeriveErr> {
        Self::from(di).into()
    }


    fn find_field_from_name(&self, name: String) -> Option<Rc<NetStructField>> {
        self.fields
            .iter()
            .find(|f| f.name == name)
            .map(|f| f.clone())
    }

    fn find_all_vec_fields(&self) -> HashMap<String, VecField> {
        fn get_vec_length(
            f: &Rc<NetStructField>,
        ) -> Option<(Rc<NetStructField>, String, SizeUnit)> {
            f.net_struct_attr.iter().find_map(|attr| match attr {
                FieldAttr::Vec {
                    vec_len_field: v,
                    unit: u,
                } => Some((f.clone(), v.clone(), u.clone())),
                _ => None,
            })
        }
        HashMap::from_iter(self.fields.iter().filter_map(|f| get_vec_length(f)).map(
            |(data_field, l_f, len_unit)| {
                (
                    data_field.name.clone(),
                    VecField {
                        _data_field: data_field,
                        len_field: self.find_field_from_name(l_f).unwrap(),
                        len_unit,
                    },
                )
            },
        ))
    }

    fn parse_attr_struct_len(&mut self, ts: &TokenStream) -> String {
        let expect_group_msg = format!(
            "Expected parenthesis with arguments after \"{}\"",
            STRUCT_SIZE_PATH
        );
        let expect_attr_name_msg = format!("Expected identifier \"{}\"", STRUCT_SIZE_PATH);
        let expect_field_name_msg = format!("Expected a field name for \"{}\"", STRUCT_SIZE_PATH);
        const NO_SUCH_FIELD_MSG: &'static str = "specified field for struct_len is not found";

        let mut it = ts.clone().into_iter();
        assert_eq!(
            expect_ident(&mut it, expect_attr_name_msg.as_str()),
            String::from(STRUCT_SIZE_PATH),
            "{}",
            expect_attr_name_msg.as_str(),
        );
        let mut arg_it = expect_group(&mut it, Delimiter::Parenthesis, expect_group_msg.as_str())
            .into_iter()
            .peekable();
        let struct_len_field_name = expect_ident(&mut arg_it, expect_field_name_msg.as_str());
        consume_punct(&mut arg_it, ',');

        let len_unit = consume_ident(&mut arg_it)
            .map(|s| SizeUnit::from(s))
            .unwrap_or(SizeUnit::BYTES);

        let net_struct_len_field = self
            .fields
            .iter()
            .find(|f| f.name == struct_len_field_name)
            .expect(NO_SUCH_FIELD_MSG);
        self.attrs.struct_len = Some((net_struct_len_field.clone(), len_unit));
        struct_len_field_name
    }
}

impl From<DeriveInput> for NetStruct {
    fn from(di: DeriveInput) -> Self {
        let Data::Struct(ds) = &di.data else {
            panic!("Expected a struct");
        };
        let mut ns = Self {
            derive_input: di.clone(),
            fields: ds
                .fields
                .iter()
                .map(|f| Rc::new(NetStructField::from(f)))
                .collect(),
            attrs: NetStructAttr { struct_len: None },
        };
        parse_attr(&di.attrs, ATTR_PATH, |tokens| {
            ns.parse_attr_struct_len(tokens);
        });
        ns
    }
}

impl Into<Result<TokenStream, DeriveErr>> for NetStruct {
    fn into(self) -> Result<TokenStream, DeriveErr> {
        let struct_name = &self.derive_input.ident;
        let mut ts = TokenStream::new();
        ts.extend(self.serializer()?);
        ts.extend(self.parser()?);
        ts.extend(self.comparer()?);
        ts.extend(quote! {impl net_struct_serde::traits::NetStruct for #struct_name {} });
        Ok(ts)
    }
}
