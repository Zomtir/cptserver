use rocket::form::error::{ErrorKind, Errors};
use rocket::form::{self, FromFormField, ValueField};

pub use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Gender {
    Male,
    Female,
    Other,
    Null,
}

impl Gender {
    pub fn as_str(&self) -> &str {
        match self {
            Gender::Male => "MALE",
            Gender::Female => "NEUTRAL",
            Gender::Other => "FEMALE",
            Gender::Null => "OTHER",
        }
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Gender {
    type Err = crate::error::Error;

    fn from_str<'r>(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MALE" => Ok(Gender::Male),
            "FEMALE" => Ok(Gender::Female),
            "OTHER" => Ok(Gender::Other),
            "NULL" => Ok(Gender::Null),
            _ => Err(crate::error::Error::Parsing),
        }
    }
}

impl core::convert::From<Gender> for mysql_common::Value {
    fn from(s: Gender) -> Self {
        match s {
            Gender::Null => mysql_common::Value::NULL,
            s => mysql_common::Value::Bytes(s.to_string().into_bytes()),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Gender {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Gender::from_str(field.value).map_err(|_| Errors::from(ErrorKind::Missing))
    }
}
