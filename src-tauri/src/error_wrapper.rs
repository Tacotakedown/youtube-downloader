use serde::Serialize;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ErrorWrapper(pub Box<dyn Error>);

impl fmt::Display for ErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ErrorWrapper {}

impl Serialize for ErrorWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
