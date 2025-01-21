use rocket::form::{self, FromFormField, ValueField};
use rocket::form::error::{Errors, ErrorKind};

pub use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Confirmation {
    Positive,
    Neutral,
    Negative,
    Null,
}

impl Confirmation {
    pub fn as_str(&self) -> &str {
        match self {
            Confirmation::Positive => "POSITIVE",
            Confirmation::Neutral => "NEUTRAL",
            Confirmation::Negative => "NEGATIVE",
            Confirmation::Null => "NULL",
        }
    }
}

impl std::fmt::Display for Confirmation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Confirmation {
    type Err = crate::error::Error;

    fn from_str<'r>(s: &str) -> Result<Self, Self::Err> {
        match s {
            "POSITIVE" => Ok(Confirmation::Positive),
            "NEUTRAL" => Ok(Confirmation::Neutral),
            "NEGATIVE" => Ok(Confirmation::Negative),
            "NULL" => Ok(Confirmation::Null),
            _ => Err(crate::error::Error::Parsing),
        }
    }
}

impl core::convert::From<Confirmation> for mysql_common::Value {
    fn from(s: Confirmation) -> Self {
        match s {
            Confirmation::Null => mysql_common::Value::NULL,
            s => mysql_common::Value::Bytes(s.to_string().into_bytes()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Confirmation {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Confirmation::from_str(field.value).map_err(|_| Errors::from(ErrorKind::Missing))
    }
}
