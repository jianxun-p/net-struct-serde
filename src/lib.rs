mod de;
mod err;
mod ser;

mod flavour;
pub mod traits;

#[derive(Debug)]
pub struct NetStructSerializer<'a> {
    buf: &'a mut [u8],
    len: usize,
}

#[derive(Debug, Clone)]
pub struct NetStructDeserializer<'a> {
    dir: bool,
    init_count: usize,
    buf: &'a [u8],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerdeErr {
    Message(String),
    Eof,
    NotEnoughSpace,
    NotSupported,
    ParseFailed,
}
