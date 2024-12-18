use core::fmt::Debug;

pub trait Flavour<D: Deserializer>: Sized + Debug {
    fn flavour<T>(val: T) -> Result<T, D::Error>;
}

pub trait Deserializer: Sized + Debug {
    type Error;

    type F: Flavour<Self>;

    fn expect(self, len: usize) -> Result<Self, Self::Error>;

    fn take<B: AsMut<[u8]>>(self, buf: &mut B) -> Result<Self, Self::Error>;

    fn skip(self, len: usize) -> Result<Self, Self::Error>;

    fn reverse(self) -> Result<Self, Self::Error>;

    fn truncate(self, len: usize) -> Result<Self, Self::Error>;

    fn finalize(self) -> Result<usize, Self::Error>;

    fn deserialize_bool(self, v: &mut bool) -> Result<Self, Self::Error>;

    fn deserialize_i8(self, v: &mut i8) -> Result<Self, Self::Error>;

    fn deserialize_i16(self, v: &mut i16) -> Result<Self, Self::Error>;

    fn deserialize_i32(self, v: &mut i32) -> Result<Self, Self::Error>;

    fn deserialize_i64(self, v: &mut i64) -> Result<Self, Self::Error>;

    fn deserialize_u8(self, v: &mut u8) -> Result<Self, Self::Error>;

    fn deserialize_u16(self, v: &mut u16) -> Result<Self, Self::Error>;

    fn deserialize_u32(self, v: &mut u32) -> Result<Self, Self::Error>;

    fn deserialize_u64(self, v: &mut u64) -> Result<Self, Self::Error>;

    fn deserialize_f32(self, v: &mut f32) -> Result<Self, Self::Error>;

    fn deserialize_f64(self, v: &mut f64) -> Result<Self, Self::Error>;

    fn deserialize_field<F: Deserialize>(
        self,
        field: &mut F,
        field_name: &'static str,
    ) -> Result<Self, Self::Error>;

    fn deserialize_seq<E: Deserialize, S: AsMut<[E]>>(self, s: S, len: usize) -> Result<Self, Self::Error>;

    fn deserialize_variant<V: Deserialize>(self, variant: &mut V) -> Result<Self, Self::Error>;
}

pub trait StructDeserializer<D: Deserializer>: Sized + Debug {
    fn deserialize_field<F: Deserialize>(
        self,
        field: &mut F,
        field_name: &'static str,
    ) -> Result<Self, D::Error>;
    fn struct_end(self) -> Result<D, D::Error>;
}

pub trait Deserialize: Sized + Debug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer;
}

pub trait NetStruct: serde::ser::Serialize + crate::traits::Deserialize + core::cmp::Eq {
    
}

