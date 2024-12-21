pub(crate) mod helper;
mod net_enum;
mod net_struct;
use net_enum::*;
use net_struct::*;
use syn::DeriveInput;
mod err;

#[cfg(test)]
mod test;

/// usage:
/// ```
///  #[derive(NetStruct)]
///  pub struct StructName {
///     field1: u8,
///     // fixed length arrays
///     arr1: [u8; 4],           
///     vec1_bytes: u16,
///     // vec with max capacity: length 8, actual size: vec1_bytes bytes
///     #[net_struct(vec_len(vec1_bytes, bytes))]
///     vec1: [u16; 8],
///     vec2_bits: u32,
///     // vec with max capacity: length 20, actual size: vec2_bits bits
///     #[net_struct(vec_len(vec2_bits, bits))]
///     vec2: [u8; 20],     
///     vec3_len: u8,
///     // vec with max capacity: length 4, actual length: vec3_len
///     #[net_struct(vec_len(vec3_len))]
///     vec3: [OtherStruct; 4],
/// }
/// ```
/// Creates an implementation of the following traits for the attached structure:
/// - `net_struct_serde::traits::NetStruct`
/// - `serde::Serialize`
/// - `net_struct_serde::traits::Deserialize`
///   - `Sized`::
/// - `core::cmp::Eq`
#[proc_macro_derive(NetStruct, attributes(net_struct))]
pub fn derive_net_struct(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let a: DeriveInput = syn::parse(item.clone()).unwrap();
    match &a.data {
        syn::Data::Struct(_) => match NetStruct::derive_input_to_token_stream(a) {
            Ok(ts) => {
                // println!("{}", &ts.to_string());
                proc_macro::TokenStream::from(ts)
            }
            Err(e) => panic!("{:?}", e),
        },
        _ => panic!("Expected a struct"),
    }
}

/// usage:
/// ```
///  #[derive(Debug, NetEnum, PartialEq, Eq, Clone, Copy)]
/// #[net_enum(repr(isize))]
/// enum TestEnum {
///     VarA = 0,
///     VarB = 10,
/// }
/// ```
/// Creates an implementation of the following traits for the attached structure:
/// - `net_struct_serde::traits::NetEnum`
/// - `serde::Serialize`
/// - `net_struct_serde::traits::Deserialize`
///   - `Sized`
#[proc_macro_derive(NetEnum, attributes(net_enum))]
pub fn derive_net_enum(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let a: DeriveInput = syn::parse(item.clone()).unwrap();
    match &a.data {
        syn::Data::Enum(_) => match NetEnum::derive_input_to_token_stream(a) {
            Ok(ts) => {
                // println!("{}", &ts.to_string());
                proc_macro::TokenStream::from(ts)
            }
            Err(e) => panic!("{:?}", e),
        },
        _ => panic!("Expected a enum"),
    }
}
