use std::fmt::Display;

#[derive(Debug)]
pub struct Error;

impl Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("alyx uni_code error")
    }
}

impl std::error::Error for Error {}
