use super::{Decoder, Encoder, Error};

pub trait Unicode: Sized {
    fn encode<En: Encoder>(&self, encoder: En) -> Result<(), Error>;
    fn decode<De: Decoder>(decoder: De) -> Result<Self, Error>;
}

impl Unicode for bool {
    fn encode<En: Encoder>(&self, encoder: En) -> Result<(), Error> {
        encoder.encode_bool(*self)?;
        Ok(())
    }

    fn decode<De: Decoder>(decoder: De) -> Result<Self, Error> {
        let bool = decoder.decode_bool()?;
        Ok(bool)
    }
}
