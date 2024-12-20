use net_struct_derive::NetStruct;
use net_struct_serde::traits::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Debug, NetStruct)]
pub struct SomeStruct {
    pub field1: u8,
    #[net_struct(vec_len(vec1_bytes, bytes))]
    pub vec1: [u16; 8],
    pub vec2_len: u16,
    #[net_struct(vec_len(vec2_len))]
    pub vec2: [u8; 8],
    pub vec1_bytes: u8,
    pub arr1: [u8; 4],
    pub field2: u8,
}

#[test]
fn reverse2() {
    const S: SomeStruct = SomeStruct {
        field1: 99,
        vec1: [4, 5, 6, 7, 8, 9, 10, 11],
        vec2_len: 3,
        vec2: [255, 254, 253, 252, 251, 250, 249, 248],
        vec1_bytes: 6,
        arr1: [13, 17, 19, 23],
        field2: 7,
    };
    const CORRECT_SERIALIZED: [u8; 18] = [99, 0, 4, 0, 5, 0, 6, 0, 3, 255, 254, 253, 6, 13, 17, 19, 23, 7];
    let mut serialized = [0u8; CORRECT_SERIALIZED.len()];
    let mut serializer = net_struct_serde::NetStructSerializer::new(&mut serialized);
    S.serialize(&mut serializer).unwrap();
    let serialized_size = serializer.finalize();
    assert_eq!(serialized, CORRECT_SERIALIZED);
    println!("serialized(DEC): {:?}", &serialized[..serialized_size]);
    println!("serialized(HEX): {:02x?}", &serialized[..serialized_size]);

    let mut deserializer = net_struct_serde::NetStructDeserializer::new(&CORRECT_SERIALIZED);
    let deserialized = SomeStruct::deserialize(&mut deserializer).unwrap();
    assert_eq!(S, deserialized);
    dbg!(deserialized);
}
