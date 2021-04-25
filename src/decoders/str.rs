use crate::{Decoder, Error};
pub struct StrDecoder<'str> {
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
fn decode_true() {
    let str = "true";
    let mut decoder = StrDecoder::new(str);
    let bool = decoder.decode_bool().unwrap();
    assert!(bool);
}

#[test]
fn decode_false() {
    let str = "false";
    let mut decoder = StrDecoder::new(str);
    let bool = decoder.decode_bool().unwrap();
    assert!(!bool);
}
