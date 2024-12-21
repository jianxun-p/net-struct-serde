use super::*;
use proc_macro2::Delimiter;
use quote::ToTokens;
use std::collections::BTreeMap;

const VEC_LEN_ATTR_PATH: &'static str = "vec_len";
const PHANTOM_ATTR_PATH: &'static str = "phantom";

static SIZE_UNIT_MAP: std::sync::OnceLock<BTreeMap<String, SizeUnit>> = std::sync::OnceLock::new();
fn size_unit_map() -> &'static BTreeMap<String, SizeUnit> {
    SIZE_UNIT_MAP.get_or_init(|| {
        BTreeMap::from([
            (String::from("len"), SizeUnit::LENGTH),
            (String::from("bytes"), SizeUnit::BYTES),
            (String::from("B"), SizeUnit::BYTES),
            (String::from("bits"), SizeUnit::BITS),
        ])
    })
}

#[derive(Clone)]
pub(super) struct NetStructField {
    pub(super) _field: syn::Field,
    pub(super) name: String,
    pub(super) net_struct_attr: Vec<FieldAttr>,
    pub(super) ty: NetStructFieldType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FieldAttr {
    Vec {
        vec_len_field: String,
        unit: SizeUnit,
    },
    Phantom,
}

#[derive(Debug, Clone)]
pub(super) enum NetStructFieldType {
    Val { ty: TokenStream },
    Arr { ty: TokenStream, capacity: String }, // fixed size array
    Vec { ty: TokenStream, capacity: String }, // vector
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(super) enum SizeUnit {
    BITS,
    BYTES,
    #[default]
    LENGTH,
}

#[derive(Clone)]
pub(super) struct VecField {
    pub(super) _data_field: Rc<NetStructField>,
    pub(super) len_field: Rc<NetStructField>,
    pub(super) len_unit: SizeUnit,
}

impl From<String> for SizeUnit {
    fn from(s: String) -> Self {
        const UNEXPECTED_UNIT_MSG: &'static str = "Unexpected size unit";
        if s.is_empty() {
            return Self::default();
        }
        match size_unit_map().get(&s) {
            Some(v) => *v,
            None => panic!("{}", UNEXPECTED_UNIT_MSG),
        }
    }
}

impl From<&syn::Field> for NetStructField {
    fn from(field: &syn::Field) -> Self {
        let name = field
            .ident
            .clone()
            .expect("Doesn't support unnamed fields")
            .to_string();
        let mut s = Self {
            _field: field.clone(),
            name,
            net_struct_attr: Vec::new(),
            ty: match &field.ty {
                syn::Type::Array(arr) => NetStructFieldType::Arr {
                    ty: arr.elem.to_token_stream(),
                    capacity: arr.len.to_token_stream().to_string(),
                },
                syn::Type::Path(ty) => NetStructFieldType::Val {
                    ty: ty.path.to_token_stream(),
                },
                _ => unimplemented!("only support Array(vector) or Path typed fields"),
            },
        };
        parse_attr(&field.attrs, ATTR_PATH, |ts| s.parse_attr_vec_len(ts));
        parse_attr(&field.attrs, ATTR_PATH, |ts| s.parse_attr_phantom(ts));
        if s.is_vec() {
            let NetStructFieldType::Arr { ty, capacity } = s.ty else {
                panic!("Since this field is a vector, expected type to have an initial value of NetStructFieldType::Arr")
            };
            s.ty = NetStructFieldType::Vec {
                ty: ty,
                capacity: capacity,
            }
        }
        s
    }
}

impl NetStructField {
    pub(super) fn is_vec(&self) -> bool {
        self.net_struct_attr
            .iter()
            .find(|attr| match attr {
                FieldAttr::Vec {
                    vec_len_field: _,
                    unit: _,
                } => true,
                _ => false,
            })
            .is_some()
    }

    pub(super) fn is_phantom(&self) -> bool {
        self.net_struct_attr
            .iter()
            .find(|attr| match attr {
                FieldAttr::Phantom => true,
                _ => false,
            })
            .is_some()
    }

    fn parse_attr_phantom(&mut self, ts: &TokenStream) {
        let phantom_attr = String::from(PHANTOM_ATTR_PATH);
        let expect_attr_name_msg = format!("Expected identifier \"{}\"", PHANTOM_ATTR_PATH);
        let mut it = ts.clone().into_iter().peekable();
        while it.peek().is_some() {
            if phantom_attr != expect_ident(&mut it, expect_attr_name_msg.as_str()) {
                it = skip_until_punct(&mut it, ',');
                if it.next().is_none() {
                    break;
                }
            }
            self.net_struct_attr.push(FieldAttr::Phantom);
        }
    }

    fn parse_attr_vec_len(&mut self, ts: &TokenStream) {
        let vec_len_attr = String::from(VEC_LEN_ATTR_PATH);
        let expect_group_msg = format!(
            "Expected parenthesis with arguments after \"{}\"",
            VEC_LEN_ATTR_PATH
        );
        let expect_attr_name_msg = format!("Expected identifier \"{}\"", VEC_LEN_ATTR_PATH);
        let expect_field_name_msg =
            format!("Expected a vector field name for \"{}\"", VEC_LEN_ATTR_PATH);

        let mut it = ts.clone().into_iter().peekable();
        while it.peek().is_some() {
            if vec_len_attr != expect_ident(&mut it, expect_attr_name_msg.as_str()) {
                it = skip_until_punct(&mut it, ',');
                if it.next().is_none() {
                    break;
                }
            }
            let mut arg_it =
                expect_group(&mut it, Delimiter::Parenthesis, expect_group_msg.as_str())
                    .into_iter()
                    .peekable();
            let vec_len_field_name = expect_ident(&mut arg_it, expect_field_name_msg.as_str());
            consume_punct(&mut arg_it, ',');
            let len_unit = consume_ident(&mut arg_it)
                .map(|s| SizeUnit::from(s))
                .unwrap_or(SizeUnit::LENGTH);
            self.net_struct_attr.push(FieldAttr::Vec {
                vec_len_field: vec_len_field_name,
                unit: len_unit,
            });
        }
    }
}
