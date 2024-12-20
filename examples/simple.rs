use net_struct_derive::NetStruct;
use net_struct_serde::traits::*;
use serde::Serialize;

#[derive(Debug, Clone, NetStruct)]
pub(self) struct SimpleStruct {
    pub x: u8,
    pub y: i8,
    #[net_struct(struct_len(x, len))]
    pub z: i32,
}

impl SimpleStruct {
    pub fn f() {
        println!("This is SimpleStruct::f");
    }
}

fn main() {
    const SIMPLE_STRUCT: SimpleStruct = SimpleStruct {
        x: 99,
        y: -19,
        z: -655367,
    };
    SimpleStruct::f();

    const CORRECT_SERIALIZED: [u8; 6] = [99u8, 237u8, 255u8, 245u8, 255u8, 249u8];
    let mut serialized = [0u8; CORRECT_SERIALIZED.len()];
    let mut serializer = net_struct_serde::NetStructSerializer::new(&mut serialized);
    SIMPLE_STRUCT.serialize(&mut serializer).unwrap();
    let serialized_size = serializer.finalize();
    assert_eq!(
        &serialized[..serialized_size],
        &CORRECT_SERIALIZED[..serialized_size]
    );
    println!("serialized: {serialized:?}");
    let mut deserializer =
        net_struct_serde::NetStructDeserializer::new(CORRECT_SERIALIZED.as_slice());
    let deserialized = SimpleStruct::deserialize(&mut deserializer).unwrap();
    assert_eq!(SIMPLE_STRUCT, deserialized);
    dbg!(deserialized);
}
