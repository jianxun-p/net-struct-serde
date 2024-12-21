use net_struct_derive::NetStruct;
use net_struct_serde::traits::*;

#[derive(Copy, Clone, Debug, NetStruct)]
pub struct OtherStruct {
    pub x: u8,
    pub y: u8,
}

#[derive(Copy, Clone, Debug, NetStruct)]
pub struct StructName {
    pub field1: u8,
    pub arr1: [u8; 4],
    pub vec1_bytes: u8,
    #[net_struct(vec_len(vec1_bytes, bytes))]
    pub vec1: [u16; 8],
    pub vec2_bits: u32,
    #[net_struct(vec_len(vec2_bits, bits))]
    pub vec2: [u8; 16],
    pub vec3_len: u8,
    #[net_struct(vec_len(vec3_len, len))]
    pub vec3: [OtherStruct; 4],
}

#[test]
fn sample() {
    const S: StructName = StructName {
        field1: 99,
        arr1: [1u8, 2u8, 3u8, 4u8],
        vec1_bytes: 6,
        vec1: [4, 5, 6, 7, 8, 9, 10, 11],
        vec2_bits: 8,
        vec2: [
            73, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
        ],
        vec3_len: 2,
        vec3: [
            OtherStruct { x: 11, y: 13 },
            OtherStruct { x: 17, y: 19 },
            OtherStruct { x: 23, y: 31 },
            OtherStruct { x: 37, y: 51 },
        ],
    };
    const CORRECT_SERIALIZED: [u8; 24] = [
        99, 1, 2, 3, 4, 6, 0, 4, 0, 5, 0, 6, 0, 0, 0, 8, 73, 2, 11, 13, 17, 19, 0, 0,
    ];
    let mut serialized = [0u8; CORRECT_SERIALIZED.len()];
    let mut serializer = net_struct_serde::NetStructSerializer::new(&mut serialized);
    S.serialize(&mut serializer).unwrap();
    let serialized_size = serializer.finalize();
    assert_eq!(serialized, CORRECT_SERIALIZED);
    println!("serialized(DEC): {:?}", &serialized[..serialized_size]);
    println!("serialized(HEX): {:02x?}", &serialized[..serialized_size]);

    let mut deserializer = net_struct_serde::NetStructDeserializer::new(&CORRECT_SERIALIZED);
    let deserialized = StructName::deserialize(&mut deserializer).unwrap();
    assert_eq!(S, deserialized);
    dbg!(deserialized);
}
