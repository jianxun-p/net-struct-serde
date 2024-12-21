use net_struct_derive::NetEnum;
use net_struct_serde::{traits::*, *};

#[derive(Debug, NetEnum, PartialEq, Eq, Clone, Copy)]
#[net_enum(repr(isize))]
enum TestEnum {
    VarA = 0,
    VarB = 10,
}

#[test]
fn simple() {
    const A: TestEnum = TestEnum::VarA;
    const A_VAL: isize = 0isize;
    const B: TestEnum = TestEnum::VarB;
    const B_VAL: isize = 10isize;
    const A_BYTES: [u8; 8] = A_VAL.to_be_bytes();
    const B_BYTES: [u8; 8] = B_VAL.to_be_bytes();
    let mut serialized_a = [0u8; A_BYTES.len()];
    let mut serialized_b = [0u8; B_BYTES.len()];
    let mut a_serializer = NetStructSerializer::new(serialized_a.as_mut_slice());
    let mut b_serializer = NetStructSerializer::new(serialized_b.as_mut_slice());
    let mut a_deserializer = NetStructDeserializer::new(A_BYTES.as_slice());
    let mut b_deserializer = NetStructDeserializer::new(B_BYTES.as_slice());
    assert_eq!(A_VAL, A.into());
    assert_eq!(B_VAL, B.into());
    assert_eq!(Ok(A), TestEnum::try_from(A_VAL));
    assert_eq!(Ok(B), TestEnum::try_from(B_VAL));
    assert_eq!(Err(SerdeErr::ParseFailed), TestEnum::try_from(100isize));
    assert_eq!(Ok(()), A.serialize(&mut a_serializer));
    assert_eq!(Ok(()), B.serialize(&mut b_serializer));
    assert_eq!(A_BYTES.len(), a_serializer.finalize());
    assert_eq!(B_BYTES.len(), b_serializer.finalize());
    assert_eq!(A_BYTES, serialized_a);
    assert_eq!(B_BYTES, serialized_b);
    assert_eq!(Ok(A), TestEnum::deserialize(&mut a_deserializer));
    assert_eq!(Ok(B), TestEnum::deserialize(&mut b_deserializer));
}
