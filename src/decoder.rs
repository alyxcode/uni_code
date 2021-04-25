use super::Error;
pub trait Decoder {
    fn decode_bool(self) -> Result<bool, Error>;
}
