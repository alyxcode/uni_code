mod error;
pub use error::Error;
mod encoder;
pub use encoder::Encoder;
mod decoder;
pub use decoder::Decoder;
mod unicode;
pub use unicode::Unicode;

mod encoders;
use encoders::StringEncoder;
mod decoders;
use decoders::StrDecoder;

pub fn encode_to_string<T: Unicode>(value: &T) -> Result<String, Error> {
    let mut encoder = StringEncoder::new();
    value.encode(&mut encoder)?;
    Ok(encoder.into_string())
}

pub fn decode_from_str<T: Unicode>(str: &str) -> Result<T, Error> {
    let mut decoder = StrDecoder::new(str);
    let value = Unicode::decode(&mut decoder)?;
    Ok(value)
}
