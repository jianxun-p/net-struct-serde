use syn::*;
use quote::quote;


#[test]
fn test_unambigious() {
    let a: DeriveInput = syn::parse(quote! {
        pub struct SomeStruct {
            pub field1: u8,
            #[net_struct(vec_len(vec1_bytes, bytes))]
            pub vec1: [u16; 8],
            pub vec2_len: u16,
            pub vec1_bytes: u8,
            pub arr1: [u8; 4],
            #[net_struct(vec_len(vec2_len))]
            pub vec2: [u8; 8],
            pub field2: u8,
        }
    }).unwrap();
    if let syn::Data::Struct(_) = a.data.clone() {
        let net_struct = NetStruct::from(a);
        assert!(net_struct.into().is_err());
    } else {
        panic!()
    }
}

