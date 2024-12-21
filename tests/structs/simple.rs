use net_struct_serde::*;

#[derive(Debug, Clone, NetStruct)]
pub(self) struct SimpleStruct {
    // pub size: u8,
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

#[test]
fn simple() {
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
    assert_eq!(serializer.finalize(), CORRECT_SERIALIZED.len());
    assert_eq!(serialized, CORRECT_SERIALIZED);
    println!("serialized: {serialized:?}");
    let mut deserializer =
        net_struct_serde::NetStructDeserializer::new(CORRECT_SERIALIZED.as_slice());
    let deserialized = SimpleStruct::deserialize(&mut deserializer).unwrap();
    assert_eq!(SIMPLE_STRUCT, deserialized);
    assert_eq!(deserializer.finalize(), CORRECT_SERIALIZED.len());
    println!("{:?}", deserialized);
}
