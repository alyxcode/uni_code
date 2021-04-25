pub fn encode_to_string<T: Unicode>(value: &T) -> Result<String, Error> {
    let mut encoder = StringEncoder::new();
    value.encode(&mut encoder)?;
    Ok(encoder.string)
}

pub fn decode_from_str<'str, T: Unicode>(str: &'str str) -> Result<T, Error> {
    let mut decoder = StrDecoder::new(str);
    let value = Unicode::decode(&mut decoder)?;
    Ok(value)
}

#[derive(Debug)]
pub struct Error;

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

pub trait Encoder {
    fn encode_bool(self, value: bool) -> Result<(), Error>;
}

pub trait Decoder {
    fn decode_bool(self) -> Result<bool, Error>;
}

struct StringEncoder {
    string: String,
}

impl StringEncoder {
    pub fn new() -> Self {
        StringEncoder {
            string: String::new(),
        }
    }
}

struct StrDecoder<'str> {
    bytes: &'str [u8],
    index: usize,
}

impl<'str> StrDecoder<'str> {
    pub fn new(str: &'str str) -> Self {
        StrDecoder {
            bytes: str.as_bytes(),
            index: 0,
        }
    }

    pub fn advance_by(&mut self, step: usize) {
        self.index += step;
    }

    pub fn ident(&mut self, ident: &[u8]) -> bool {
        let len = ident.len();
        if self.index + len <= self.bytes.len() {
            &self.bytes[self.index..self.index + len] == ident
        } else {
            false
        }
    }
}

impl<'en> Encoder for &'en mut StringEncoder {
    fn encode_bool(self, value: bool) -> Result<(), Error> {
        if value {
            self.string.push_str("true");
        } else {
            self.string.push_str("false");
        }
        Ok(())
    }
}

impl<'de, 'str> Decoder for &'de mut StrDecoder<'str> {
    fn decode_bool(self) -> Result<bool, Error> {
        if self.ident(b"true") {
            self.advance_by("true".len());
            Ok(true)
        } else if self.ident(b"false") {
            self.advance_by("false".len());
            Ok(false)
        } else {
            Err(Error)
        }
    }
}

#[test]
fn encode_true_to_string() {
    let bool = true;
    let string = encode_to_string(&bool).unwrap();
    assert_eq!(string, "true".to_string());
}

#[test]
fn encode_false_to_string() {
    let bool = false;
    let string = encode_to_string(&bool).unwrap();
    assert_eq!(string, "false".to_string());
}

#[test]
fn decode_true_from_str() {
    let str = "true";
    let bool: bool = decode_from_str(str).unwrap();
    assert!(bool);
}

#[test]
fn decode_false_from_str() {
    let str = "false";
    let bool: bool = decode_from_str(str).unwrap();
    assert!(!bool);
}
