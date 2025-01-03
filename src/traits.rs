pub use serde::{Serialize, Serializer};

use crate::SerdeErr;

/// A mapping from a deserialized value to a valid value
pub trait Flavour<D: Deserializer>: Sized {
    fn flavour<T>(val: T) -> Result<T, SerdeErr>;
}

/// The `net_struct_serde::traits::Deserializer` is different from `serde::Deserializer`
/// since the `serde::Deserializer` trait does not provide the ability to deserialize fields
/// of a structure in reverse order, which is required for the case where the length field is declared
/// after the vector field. For example:
/// ```
/// use net_struct_derive::*;
/// #[derive(NetStruct)]
/// pub struct SomeStruct {
///     #[net_struct(vec_len(vec1_bytes, bytes))]
///     pub vec1: [u16; 8],
///     pub vec1_bytes: u8,
/// }
/// ```
pub trait Deserializer: Sized {
    type F: Flavour<Self>;

    /// check if there is at least `len` bytes ahead
    fn expect(self, len: usize) -> Result<Self, SerdeErr>;

    /// take `len` bytes
    fn take<B: AsMut<[u8]>>(self, buf: &mut B) -> Result<Self, SerdeErr>;

    /// skip `len` bytes
    fn skip(self, len: usize) -> Result<Self, SerdeErr>;

    /// reverse direction of the deserializer
    fn reverse(self) -> Result<Self, SerdeErr>;

    /// keep only `len` bytes
    fn truncate(self, len: usize) -> Result<Self, SerdeErr>;

    /// returns the number of bytes deserialized
    fn finalize(self) -> usize;

    fn deserialize_bool(self, v: &mut bool) -> Result<Self, SerdeErr>;

    fn deserialize_i8(self, v: &mut i8) -> Result<Self, SerdeErr>;

    fn deserialize_i16(self, v: &mut i16) -> Result<Self, SerdeErr>;

    fn deserialize_i32(self, v: &mut i32) -> Result<Self, SerdeErr>;

    fn deserialize_i64(self, v: &mut i64) -> Result<Self, SerdeErr>;

    fn deserialize_u8(self, v: &mut u8) -> Result<Self, SerdeErr>;

    fn deserialize_u16(self, v: &mut u16) -> Result<Self, SerdeErr>;

    fn deserialize_u32(self, v: &mut u32) -> Result<Self, SerdeErr>;

    fn deserialize_u64(self, v: &mut u64) -> Result<Self, SerdeErr>;

    fn deserialize_f32(self, v: &mut f32) -> Result<Self, SerdeErr>;

    fn deserialize_f64(self, v: &mut f64) -> Result<Self, SerdeErr>;

    fn deserialize_field<F: Deserialize>(
        self,
        field: &mut F,
        field_name: &'static str,
    ) -> Result<Self, SerdeErr>;

    fn deserialize_seq<E: Deserialize, S: AsMut<[E]>>(
        self,
        s: S,
        len: usize,
    ) -> Result<Self, SerdeErr>;

    fn deserialize_seq_until_end<E: Deserialize, S: AsMut<[E]>>(
        self,
        s: S,
        len: &mut usize,
        len_adj: impl Fn(usize) -> usize,
    ) -> Result<Self, SerdeErr>;

    fn deserialize_variant<V: Deserialize>(self, variant: &mut V) -> Result<Self, SerdeErr>;
}

pub trait StructDeserializer<D: Deserializer>: Sized {
    fn deserialize_field<F: Deserialize>(
        self,
        field: &mut F,
        field_name: &'static str,
    ) -> Result<Self, SerdeErr>;
    fn struct_end(self) -> Result<D, SerdeErr>;
}

/// Specifies the rules of how the structure is deserialized.
/// Note that the `Deserialize` trait is different from `serde::Deserialize`.
pub trait Deserialize: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, SerdeErr>
    where
        D: Deserializer;
}

/// `Serialize` and `Deserialize` for network protocols structures.
pub trait NetStruct: crate::traits::Serialize + crate::traits::Deserialize + core::cmp::Eq {}

/// `Serialize` and `Deserialize` for network protocols enumerations.
pub trait NetEnum:
    Clone + crate::traits::Serialize + crate::traits::Deserialize + core::cmp::Eq
{
}
