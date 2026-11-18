use crate::error::ParseError;
use std::str::FromStr;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sticker {
    pub value: String,
}

impl FromStr for Sticker {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Sticker, ParseError> {
        let mut parts = s.splitn(2, '=');
        match (parts.next(), parts.next()) {
            (Some(_), Some(value)) => Ok(Sticker { value: value.to_owned() }),
            _ => Err(ParseError::BadValue(s.to_owned())),
        }
    }
}
