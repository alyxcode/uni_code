use crate::{Encoder, Error};
pub struct StringEncoder {
    string: String,
}

impl StringEncoder {
    pub fn new() -> Self {
        StringEncoder {
            string: String::new(),
        }
    }

    pub fn into_string(self) -> String {
        self.string
    }
}

impl Default for StringEncoder {
    fn default() -> Self {
        Self::new()
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

#[test]
fn encode_true() {
    let mut encoder = StringEncoder::new();
    let bool = true;
    encoder.encode_bool(bool).unwrap();
    assert_eq!(encoder.into_string(), "true".to_string());
}

#[test]
fn encode_false() {
    let mut encoder = StringEncoder::new();
    let bool = false;
    encoder.encode_bool(bool).unwrap();
    assert_eq!(encoder.into_string(), "false".to_string());
}
