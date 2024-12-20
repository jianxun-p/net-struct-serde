pub(crate) mod helper;
mod net_struct;
use net_struct::*;
use syn::DeriveInput;
mod err;

#[cfg(test)]
mod test;

/**
 * usage:
 *  #[derive(NetStruct)]
 *  pub struct StructName {
 *      field1: u8,
 *      // fixed length arrays
 *      arr1: [u8; 4],           
 *      vec1_bytes: u16,
 *      // vec with max capacity: length 8, actual size: vec1_bytes bytes
 *      #[net_struct(vec_len(vec1_bytes, bytes))]
 *      vec1: [u16; 8],
 *      vec2_bits: u32,
 *      // vec with max capacity: length 20, actual size: vec2_bits bits
 *      #[net_struct(vec_len(vec2_bits, bits))]
 *      vec2: [u8; 20],     
 *      vec3_len: u8,
 *      // vec with max capacity: length 4, actual length: vec3_len
 *      #[net_struct(vec_len(vec3_len))]
 *      vec3: [OtherStruct; 4],
 *  }
 */
#[proc_macro_derive(NetStruct, attributes(net_struct))]
pub fn derive_net_struct(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let a: DeriveInput = syn::parse(item.clone()).unwrap();
    match a.data.clone() {
        syn::Data::Struct(_) => {
            let net_struct = NetStruct::from(a);
            match net_struct.into() {
                Ok(ts) => {
                    // println!("{}", &ts.to_string());
                    proc_macro::TokenStream::from(ts)
                }
                Err(e) => match e {
                    err::DeriveErr::AmbigiousDeserialize(msg) => panic!(
                        "Error: Ambigiouity with the Deserialize implementation {}",
                        msg
                    ),
                    err::DeriveErr::Custoum(msg) => panic!(
                        "Error: {}",
                        msg
                    ),
                },
            }
        }
        syn::Data::Union(_) => unimplemented!("No support for union typed"),
        _ => todo!("Only supports Struct typed for now"),
    }
}
