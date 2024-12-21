use crate::{
    traits::{Deserializer, Flavour},
    SerdeErr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoFlavour<D: Deserializer>(core::marker::PhantomData<D>);

impl<D: Deserializer> Flavour<D> for NoFlavour<D> {
    #[inline]
    fn flavour<T>(val: T) -> Result<T, SerdeErr> {
        Ok(val)
    }
}
