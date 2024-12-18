use crate::SerdeErr;

impl serde::ser::Error for SerdeErr {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        Self::Message(msg.to_string())
    }
}

impl serde::de::Error for SerdeErr {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        Self::Message(msg.to_string())
    }
}

impl core::fmt::Display for SerdeErr {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Message(msg) => formatter.write_str(msg.as_str()),
            Self::NotEnoughSpace => formatter.write_str("not enough buffer space"),
            Self::NotSupported => formatter.write_str("not supported"),
            Self::Eof => formatter.write_str("unexpected end of file"),
            Self::ParseFailed => formatter.write_str("failed to deserialize"),
            /* and so forth */
        }
    }
}

impl core::error::Error for SerdeErr {}
