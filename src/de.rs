use crate::{
    flavour::NoFlavour,
    traits::{Deserialize, Deserializer, Flavour, StructDeserializer},
    NetStructDeserializer, SerdeErr,
};

impl<'a> NetStructDeserializer<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self {
            dir: true,
            init_count: buf.len(),
            buf: buf,
        }
    }
}

macro_rules! deserialize_primty {
    ($s:ident, $primty:ty, $v:ident) => {
        const SIZE: usize = core::mem::size_of::<$primty>();
        $s = $s.expect(SIZE)?;
        let mut arr = core::mem::MaybeUninit::<[u8; SIZE]>::uninit();
        let buf;
        unsafe {
            $s = $s.take(&mut *arr.as_mut_ptr())?;
            buf = arr.assume_init();
        }
        let mut nsd = NetStructDeserializer {
            dir: true,
            init_count: SIZE,
            buf: buf.as_slice(),
        };
        *$v = <$primty as Deserialize>::deserialize(&mut nsd)?;
        return match nsd.finalize()? == SIZE {
            true => Ok($s),
            false => Err(SerdeErr::ParseFailed),
        }
    };
}

impl Deserializer for &mut NetStructDeserializer<'_> {
    type F = NoFlavour<Self>;

    #[inline]
    fn expect(self, len: usize) -> Result<Self, SerdeErr> {
        match self.buf.len() >= len {
            true => Ok(self),
            false => Err(SerdeErr::Eof),
        }
    }

    #[inline]
    fn take<B: AsMut<[u8]>>(mut self, buf: &mut B) -> Result<Self, SerdeErr> {
        let b = buf.as_mut();
        self = self.expect(b.len())?;
        let (b1, b2) = match self.dir {
            true => (&self.buf[..b.len()], &self.buf[b.len()..]),
            false => (
                &self.buf[(self.buf.len() - b.len())..],
                &self.buf[..(self.buf.len() - b.len())],
            ),
        };
        b.copy_from_slice(b1);
        self.buf = b2;
        Ok(self)
    }

    #[inline]
    fn truncate(self, len: usize) -> Result<Self, SerdeErr> {
        match self.buf.len() >= len {
            true => {
                self.buf = match self.dir {
                    true => &self.buf[..len],
                    false => &self.buf[(self.buf.len() - len)..],
                };
                Ok(self)
            }
            false => Err(SerdeErr::Eof),
        }
    }

    #[inline]
    fn skip(mut self, len: usize) -> Result<Self, SerdeErr> {
        self = self.expect(len)?;
        self.buf = match self.dir {
            true => &self.buf[len..],
            false => &self.buf[..(self.buf.len() - len)],
        };
        self.buf = &self.buf[len..];
        Ok(self)
    }

    #[inline]
    fn reverse(self) -> Result<Self, SerdeErr> {
        self.dir = !self.dir;
        Ok(self)
    }

    #[inline]
    fn finalize(self) -> Result<usize, SerdeErr> {
        Ok(self.init_count - self.buf.len())
    }

    #[inline]
    fn deserialize_bool(mut self, v: &mut bool) -> Result<Self, SerdeErr> {
        const SIZE: usize = core::mem::size_of::<bool>();
        let mut arr = [0u8; SIZE];
        self = self.take(&mut arr)?;
        *v = Self::F::flavour(arr.iter().fold(false, |acc, &i| acc || i != 0u8))?;
        Ok(self)
    }

    #[inline]
    fn deserialize_i8(mut self, v: &mut i8) -> Result<Self, SerdeErr> {
        deserialize_primty!(self, i8, v);
    }

    #[inline]
    fn deserialize_i16(mut self, v: &mut i16) -> Result<Self, SerdeErr> {
        deserialize_primty!(self, i16, v);
    }

    #[inline]
    fn deserialize_i32(mut self, v: &mut i32) -> Result<Self, SerdeErr> {
        deserialize_primty!(self, i32, v);
    }

    #[inline]
    fn deserialize_i64(mut self, v: &mut i64) -> Result<Self, SerdeErr> {
        deserialize_primty!(self, i64, v);
    }

    #[inline]
    fn deserialize_u8(mut self, v: &mut u8) -> Result<Self, SerdeErr> {
        deserialize_primty!(self, u8, v);
    }

    #[inline]
    fn deserialize_u16(mut self, v: &mut u16) -> Result<Self, SerdeErr> {
        deserialize_primty!(self, u16, v);
    }

    #[inline]
    fn deserialize_u32(mut self, v: &mut u32) -> Result<Self, SerdeErr> {
        deserialize_primty!(self, u32, v);
    }

    #[inline]
    fn deserialize_u64(mut self, v: &mut u64) -> Result<Self, SerdeErr> {
        deserialize_primty!(self, u64, v);
    }

    #[inline]
    fn deserialize_f32(mut self, v: &mut f32) -> Result<Self, SerdeErr> {
        deserialize_primty!(self, f32, v);
    }

    #[inline]
    fn deserialize_f64(mut self, v: &mut f64) -> Result<Self, SerdeErr> {
        deserialize_primty!(self, f64, v);
    }

    #[inline]
    fn deserialize_field<E: Deserialize>(
        self,
        field: &mut E,
        _field_name: &'static str,
    ) -> Result<Self, SerdeErr> {
        *field = <E as Deserialize>::deserialize(&mut *self)?;
        Ok(self)
    }

    fn deserialize_seq<E: Deserialize, S: AsMut<[E]>>(
        self,
        mut s: S,
        len: usize,
    ) -> Result<Self, SerdeErr> {
        let arr = s.as_mut();
        if arr.len() < len {
            return Err(SerdeErr::Eof);
        }
        match self.dir {
            true => {
                for i in 0..len {
                    arr[i] = E::deserialize(&mut *self)?;
                }
            }
            false => {
                for i in (0..len).rev() {
                    arr[i] = E::deserialize(&mut *self)?;
                }
            }
        };
        Ok(self)
    }

    fn deserialize_seq_until_end<E: Deserialize, S: AsMut<[E]>>(
        self,
        mut s: S,
        len: &mut usize,
        len_adj: impl Fn(usize) -> usize,
    ) -> Result<Self, SerdeErr> {
        *len = 0;
        let arr = s.as_mut();
        while arr.len() >= *len + 1 {
            if let Ok(val) = E::deserialize(&mut *self) {
                arr[*len] = val;
                *len += 1;
            } else {
                break;
            }
        }
        *len = len_adj(*len);
        Ok(self)
    }

    #[inline]
    fn deserialize_variant<V: Deserialize>(self, v: &mut V) -> Result<Self, SerdeErr> {
        *v = V::deserialize(&mut *self)?;
        Ok(self)
    }
}

impl<'a, 'b: 'a> StructDeserializer<&'a mut NetStructDeserializer<'b>>
    for &'a mut NetStructDeserializer<'b>
{
    #[inline]
    fn deserialize_field<E: Deserialize>(
        self,
        field: &mut E,
        _field_name: &'static str,
    ) -> Result<Self, SerdeErr> {
        *field = <E as Deserialize>::deserialize(&mut *self)?;
        Ok(self)
    }

    #[inline]
    fn struct_end(self) -> Result<Self, SerdeErr> {
        Ok(self)
    }
}

macro_rules! impl_deserialize_for_primty {
    ($primty:ty) => {
        impl Deserialize for $primty {
            fn deserialize<D>(deserializer: D) -> Result<Self, SerdeErr>
            where
                D: Deserializer,
            {
                const SIZE: usize = core::mem::size_of::<$primty>();
                unsafe {
                    let mut arr = core::mem::MaybeUninit::<[u8; SIZE]>::uninit();
                    let _de = deserializer.take(&mut *arr.as_mut_ptr())?;
                    let v = <$primty>::from_be_bytes(arr.assume_init());
                    Ok(D::F::flavour(v)?)
                }
            }
        }
    };
}

impl_deserialize_for_primty!(i8);
impl_deserialize_for_primty!(i16);
impl_deserialize_for_primty!(i32);
impl_deserialize_for_primty!(i64);
impl_deserialize_for_primty!(i128);
impl_deserialize_for_primty!(isize);
impl_deserialize_for_primty!(u8);
impl_deserialize_for_primty!(u16);
impl_deserialize_for_primty!(u32);
impl_deserialize_for_primty!(u64);
impl_deserialize_for_primty!(u128);
impl_deserialize_for_primty!(usize);
impl_deserialize_for_primty!(f32);
impl_deserialize_for_primty!(f64);

impl<T: Deserialize> Deserialize for Option<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, SerdeErr>
    where
        D: Deserializer,
    {
        match deserializer.expect(1) {
            Ok(deserializer) => Ok(Some(T::deserialize(deserializer)?)),
            Err(_) => Ok(None),
        }
    }
}

#[cfg(test)]
mod test {
    use super::NetStructDeserializer;
    use crate::traits::*;
    use crate::SerdeErr;

    #[test]
    fn primint1() {
        let a: [u8; 2] = [0x01, 0x02];
        let mut nsd = NetStructDeserializer::new(a.as_slice());
        assert_eq!(u16::deserialize(&mut nsd), Ok(0x0102));
    }

    #[test]
    fn struct1() {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct St {
            x: i32,
            y: i16,
        }
        impl Deserialize for St {
            fn deserialize<D>(deserializer: D) -> Result<Self, SerdeErr>
            where
                D: Deserializer,
            {
                let mut s = core::mem::MaybeUninit::<St>::uninit();
                unsafe {
                    deserializer
                        .deserialize_i32(&mut (*s.as_mut_ptr()).x)?
                        .deserialize_i16(&mut (*s.as_mut_ptr()).y)?;
                    Ok(s.assume_init())
                }
            }
        }
        let a: [u8; 6] = [0x00, 0x00, 0x00, 0x01, 0x00, 0x02];
        let mut nsd = NetStructDeserializer::new(a.as_slice());
        assert_eq!(St::deserialize(&mut nsd), Ok(St { x: 1, y: 2 }));
    }

    #[test]
    fn struct2() {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct St {
            l: i32,
            arr: [i8; 8],
        }
        impl Deserialize for St {
            fn deserialize<D>(deserializer: D) -> Result<Self, SerdeErr>
            where
                D: Deserializer,
            {
                let mut s = core::mem::MaybeUninit::<St>::uninit();
                unsafe {
                    deserializer
                        .deserialize_i32(&mut (*s.as_mut_ptr()).l)?
                        .deserialize_seq(
                            &mut (*s.as_mut_ptr()).arr,
                            (*s.as_mut_ptr()).l as usize,
                        )?;
                    Ok(s.assume_init())
                }
            }
        }
        let a: [u8; 6] = [0x00, 0x00, 0x00, 0x02, 0x03, 0x07];
        let mut nsd = NetStructDeserializer::new(a.as_slice());
        assert_eq!(
            St::deserialize(&mut nsd),
            Ok(St {
                l: 2,
                arr: [3, 7, 0, 0, 0, 0, 0, 0]
            })
        );
    }

    #[test]
    fn struct3() {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct St {
            x: u8,
            y: i16,
            arr: [i8; 8],
            l: i32,
        }
        impl Deserialize for St {
            fn deserialize<D>(deserializer: D) -> Result<Self, SerdeErr>
            where
                D: Deserializer,
            {
                let mut s = core::mem::MaybeUninit::<St>::uninit();
                unsafe {
                    deserializer
                        .deserialize_u8(&mut (*s.as_mut_ptr()).x)?
                        .reverse()?
                        .deserialize_i32(&mut (*s.as_mut_ptr()).l)?
                        .deserialize_seq(&mut (*s.as_mut_ptr()).arr, (*s.as_mut_ptr()).l as usize)?
                        .reverse()?
                        .deserialize_i16(&mut (*s.as_mut_ptr()).y)?;
                    Ok(s.assume_init())
                }
            }
        }
        let a = [21, 0, 11, 0x03, 0x05, 0x07, 0x00, 0x00, 0x00, 0x03];
        let mut nsd = NetStructDeserializer::new(a.as_slice());
        assert_eq!(
            St::deserialize(&mut nsd),
            Ok(St {
                x: 21,
                y: 11,
                l: 3,
                arr: [3, 5, 7, 0, 0, 0, 0, 0]
            })
        );
    }

    #[test]
    fn seq1() {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct St {
            arr: [i32; 3],
        }
        impl Deserialize for St {
            fn deserialize<D>(deserializer: D) -> Result<Self, SerdeErr>
            where
                D: Deserializer,
            {
                let mut st = core::mem::MaybeUninit::<St>::uninit();
                unsafe {
                    deserializer.deserialize_seq(&mut (*st.as_mut_ptr()).arr, 3)?;
                    Ok(st.assume_init())
                }
            }
        }
        let a: [u8; 12] = [
            0x00, 0x00, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x04,
        ];
        let mut nsd = NetStructDeserializer::new(a.as_slice());
        assert_eq!(
            St::deserialize(&mut nsd),
            Ok(St {
                arr: [1, 0x020000, 0x030004]
            })
        );
    }

    #[test]
    fn option1() {
        let a: [u8; 2] = [0x01, 0x02];
        let mut nsd = NetStructDeserializer::new(a.as_slice());
        assert_eq!(Option::<u16>::deserialize(&mut nsd), Ok(Some(0x0102)));
        let b: [u8; 0] = [];
        nsd = NetStructDeserializer::new(b.as_slice());
        assert_eq!(Option::<u16>::deserialize(&mut nsd), Ok(None));
    }

    #[test]
    fn eof1() {
        let a: [u8; 2] = [0x01, 0x02];
        let mut nsd = NetStructDeserializer::new(a.as_slice());
        assert_eq!(u32::deserialize(&mut nsd), Err(SerdeErr::Eof));
    }
}
