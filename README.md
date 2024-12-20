# Serializer and Deserializer for Network Protocols Structures

## Serialize and Deserialize

Specifies the rules of how the structure is serialized and deserialized. Note that the `Deserialize` trait is different from `serde::Deserialize`.

## Derive NetStruct

Creates an implementation of the following traits for the attached structure:

- `NetStruct`
  - `serde::Serialize`
  - `net_struct_serde::traits::Deserialize`
    - `Sized`
  - `core::cmp::Eq`

## Serializer and Deserializer

The `NetStructSerializer` is just as simple serializer that implements the `serde::Serializer` trait.
It serializes the fields, of any structure that implements `serde::Serialize`, in declarationg ordering into big-endian bytes.

The `NetStructDeserializer` on the other hand, implements a slightly trait. It implements the `net_struct_serde::traits::Deserializer` instead of `serde::Deserializer` since the `Deserializer` trait does not provide the ability to deserialize fields of a structure in reverse order, which is required for the case where the length field is declared after the vector field. For example:

```rust
#[derive(NetStruct)]
pub struct SomeStruct {
    #[net_struct(vec_len(vec1_bytes, bytes))]
    pub vec1: [u16; 8],
    pub vec1_bytes: u8,
}
```

## Examples

Examples can be found under the `examples` and `tests` directories.
