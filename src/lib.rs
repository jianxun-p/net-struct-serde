//! This is crate implements a `Serializer` and a `Deserializer` for network protocols structures.
//! It also provides derive macro for `NetStruct` which implements the necessary traits to be serialized
//! and deserialized into/from big-endian bytes.
//!
//! # Example
//! ```
//! use net_struct_serde::{traits::*, *};
//! use net_struct_derive::*;
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
//! assert_eq!(
//!     &serialized[..serialized_size],
//!     &CORRECT_SERIALIZED[..serialized_size]
//! );
//! let mut deserializer =
//!     net_struct_serde::NetStructDeserializer::new(CORRECT_SERIALIZED.as_slice());
//! let deserialized = SimpleStruct::deserialize(&mut deserializer).unwrap();
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
//!   - `PRIMITIVE_INTEGER_TYPE`: a primitive integer type that the enumeration is serialized/deserialized into/from

mod de;
mod err;
mod ser;

mod flavour;
pub mod traits;
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
