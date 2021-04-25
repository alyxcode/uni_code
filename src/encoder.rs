use super::Error;
pub trait Encoder {
    fn encode_bool(self, value: bool) -> Result<(), Error>;
}
