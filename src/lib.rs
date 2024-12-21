//! This is crate implements a `Serializer` and a `Deserializer` for network protocols structures.
//! It also provides derive macro for `NetStruct` which implements the necessary traits to be serialized
//! and deserialized into/from big-endian bytes.
//!
//! # Example
//! ```
//! use net_struct_serde::{traits::*, *};
//! #[derive(Debug, Clone, Copy, PartialEq, Eq, NetEnum)]
//! #[net_enum(repr(u8))]
//! pub enum SimpleEnum {
//!     A = 0xff, B = 0xfe, C = 0xfd,
//! }
//! #[derive(Debug, Clone, NetStruct)]
//! pub struct SimpleStruct {
//!     pub x: u8,
//!     pub y: SimpleEnum,
//!     pub z: i32,
//! }
//! const SIMPLE_STRUCT: SimpleStruct = SimpleStruct {
//!     x: 99,
//!     y: SimpleEnum::A,
//!     z: -655367,
//! };
//! const CORRECT_SERIALIZED: [u8; 6] = [99u8, 255u8, 255u8, 245u8, 255u8, 249u8];
//! let mut serialized = [0u8; CORRECT_SERIALIZED.len()];
//! let mut serializer = net_struct_serde::NetStructSerializer::new(&mut serialized);
//! SIMPLE_STRUCT.serialize(&mut serializer).unwrap();
//! let serialized_size = serializer.finalize();
//! assert_eq!(serialized_size, CORRECT_SERIALIZED.len());
//! assert_eq!(serialized, CORRECT_SERIALIZED);
//! let mut deserializer =
//!     net_struct_serde::NetStructDeserializer::new(CORRECT_SERIALIZED.as_slice());
//! let deserialized = SimpleStruct::deserialize(&mut deserializer).unwrap();
//! let deserialized_size = deserializer.finalize();
//! assert_eq!(deserialized_size, CORRECT_SERIALIZED.len());
//! assert_eq!(SIMPLE_STRUCT, deserialized);
//! ```
//!
//! # NetStruct
//!
//! ## Field Attributes
//! The \<ARGUMENTS\> are seperated by a comma.
//! All field attributes are in the form `#[net_struct(<FIELD_ATTR>)]`:
//! - `vec_len(<VECTOR_LENGTH_FIELD>, <OPTIONAL:LENGTH_UNIT>])`
//!   - `VECTOR_LENGTH_FIELD`: a field that holds the length of the vector
//!   - `LENGTH_UNIT`: length specified in the `VECTOR_LENGTH_FIELD` has a unit:
//!     - `B` or `bytes`: in Bytes
//!     - `bits`: in bits
//!     - `len`: in number of elements (this is also the default if LENGTH_UNIT is not specified)
//! - `phantom`
//!   - a placeholder that will not be serialized, deserialized nor compared
//!
//! # NetEnum
//!
//! ## Field Attributes
//! The \<ARGUMENTS\> are seperated by a comma.
//! All field attributes are in the form `#[net_enum(<FIELD_ATTR>)]`:
//! - `repr(<PRIMITIVE_INTEGER_TYPE>])`
//!   - `PRIMITIVE_INTEGER_TYPE`: a primitive integer type that the enumeration is serialized/deserialized into/from,
//!     it is not nesscarily the same type as it is stored in memory (for that, `#[repr(<TYPE_IN_MEMORY>)]` is needed)

mod de;
mod err;
mod ser;

mod flavour;
pub mod traits;
pub use net_struct_derive::{NetEnum, NetStruct};
pub use traits::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct NetStructSerializer<'a> {
    buf: &'a mut [u8],
    len: usize,
}

#[derive(Debug, Clone)]
pub struct NetStructDeserializer<'a> {
    dir: bool,
    init_count: usize,
    buf: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerdeErr {
    Message(String),
    Eof,
    NotEnoughSpace,
    NotSupported,
    ParseFailed,
}

#[cfg(feature = "heapless")]
/// Serializes the given input into `heapless::Vec`
/// ```
/// use net_struct_serde::*;
/// #[derive(Copy, Clone, Debug, NetStruct)]
/// pub struct StructName {
///     pub field1: u16,
///     pub arr1: [u8; 4],
///     pub vec1_bytes: u8,
///     #[net_struct(vec_len(vec1_bytes, bytes))]
///     pub vec1: [u16; 8],
///     pub vec2_bits: u32,
///     #[net_struct(vec_len(vec2_bits, bits))]
///     pub vec2: [u8; 16],
/// }
/// const S: StructName = StructName {
///     field1: 99,
///     arr1: [1u8, 2u8, 3u8, 4u8],
///     vec1_bytes: 6,
///     vec1: [4, 5, 6, 7, 8, 9, 10, 11],
///     vec2_bits: 8,
///     vec2: [
///         73, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
///     ],
/// };
/// const CORRECT_SERIALIZED: [u8; 18] = [
///     0, 99, 1, 2, 3, 4, 6, 0, 4, 0, 5, 0, 6, 0, 0, 0, 8, 73,
/// ];
/// let res = to_vec::<32, StructName>(&S);
/// assert!(res.is_ok());
/// assert_eq!(
///     res.unwrap().into_array::<{CORRECT_SERIALIZED.len()}>(),
///     Ok(CORRECT_SERIALIZED)
/// );
/// ```
pub fn to_vec<const N: usize, T>(value: &T) -> Result<heapless::Vec<u8, N>, SerdeErr>
where
    T: Serialize,
{
    use heapless::Vec;
    let mut v = Vec::new();
    unsafe {
        v.set_len(N);
        let mut serializer = NetStructSerializer::new(v.as_mut_slice());
        value.serialize(&mut serializer)?;
        let serialized_len = serializer.finalize();
        v.set_len(serialized_len);
    }
    Ok(v)
}

#[inline]
/// Deserialize from the input bytes
pub fn from_slice<T: Deserialize>(data: impl AsRef<[u8]>) -> Result<T, SerdeErr> {
    let mut deserializer = NetStructDeserializer::new(data.as_ref());
    T::deserialize(&mut deserializer)
}
