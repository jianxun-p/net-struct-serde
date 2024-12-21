use std::rc::Rc;

use quote::quote;
use syn::{Data, DeriveInput};
use variants::NetEnumVariants;
use crate::{err::DeriveErr, helper::*};
use proc_macro2::*;
mod variants;
mod impl_into;
mod impl_tryfrom;
mod impl_ser;
mod impl_de;

const ATTR_PATH: &'static str = "net_enum";
const DISCRIMINANT_TYPE_PATH: &'static str = "repr";

#[derive(Clone)]
pub(super) struct NetEnum {
    derive_input: DeriveInput,
    variants: Vec<Rc<NetEnumVariants>>,
    attrs: NetEnumAttr,
}

#[derive(Clone)]
struct NetEnumAttr {
    repr: TokenStream,
}


impl From<DeriveInput> for NetEnum {
    fn from(di: DeriveInput) -> Self {
        let Data::Enum(ds) = &di.data else {
            panic!("Expected a enumeration");
        };
        let mut ns = Self {
            derive_input: di.clone(),
            variants: ds
                .variants
                .iter()
                .map(|f| Rc::new(NetEnumVariants::from(f)))
                .collect(),
            attrs: NetEnumAttr { repr: TokenStream::new() },
        };
        parse_attr(&di.attrs, ATTR_PATH, |tokens| {
            ns.parse_attr_discriminant_size(tokens);
        });
        ns
    }
}

impl NetEnum {

    pub fn derive_input_to_token_stream(di: DeriveInput) -> Result<TokenStream, DeriveErr> {
        Self::from(di).into()
    }

    fn parse_attr_discriminant_size(&mut self, ts: &TokenStream) {
        let expect_attr_ident_msg = format!(
            "Expected identifier \"{}\" for the type for discriminant", 
            DISCRIMINANT_TYPE_PATH
        );
        let expect_group_msg = format!(
            "Expected parenthesis immediately after the identifier \"{}\" for the type for discriminant", 
            DISCRIMINANT_TYPE_PATH
        );
        let mut it = ts.clone().into_iter().peekable();        
        assert_eq!(
            String::from(DISCRIMINANT_TYPE_PATH), 
            expect_ident(&mut it, expect_attr_ident_msg.as_str()), 
            "{}", expect_attr_ident_msg
        );
        self.attrs.repr = expect_group(&mut it, Delimiter::Parenthesis, expect_group_msg.as_str());
    }
}

impl Into<Result<TokenStream, DeriveErr>> for NetEnum {
    fn into(self) -> Result<TokenStream, DeriveErr> {
        let mut ts = TokenStream::new();
        let enum_name = &self.derive_input.ident;
        ts.extend(self.impl_into()?);
        ts.extend(self.impl_tryfrom()?);
        ts.extend(self.impl_serialize()?);
        ts.extend(self.impl_deserialize()?);
        ts.extend(quote! {
            impl NetEnum for #enum_name {}
        });
        Ok(ts)
    }
}
