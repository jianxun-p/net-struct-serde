use net_struct_serde::*;

#[derive(Copy, Clone, Debug, NetStruct)]
pub struct StructName {
    pub field1: u16,
    pub arr1: [u8; 4],
    pub vec1_bytes: u8,
    #[net_struct(vec_len(vec1_bytes, bytes))]
    pub vec1: [u16; 8],
    pub vec2_bits: u32,
    #[net_struct(vec_len(vec2_bits, bits))]
    pub vec2: [u8; 16],
}

#[test]
fn test_to_vec() {
    const S: StructName = StructName {
        field1: 99,
        arr1: [1u8, 2u8, 3u8, 4u8],
        vec1_bytes: 6,
        vec1: [4, 5, 6, 7, 8, 9, 10, 11],
        vec2_bits: 8,
        vec2: [
            73, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75, 75,
        ],
    };
    const CORRECT_SERIALIZED: [u8; 18] = [0, 99, 1, 2, 3, 4, 6, 0, 4, 0, 5, 0, 6, 0, 0, 0, 8, 73];
    let res = to_vec::<32, StructName>(&S);
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().into_array::<{ CORRECT_SERIALIZED.len() }>(),
        Ok(CORRECT_SERIALIZED)
    );
}
