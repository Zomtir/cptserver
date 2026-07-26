use serde::{Deserialize, Deserializer};

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
    type Err = crate::error::ErrorKind;

    fn from_str<'r>(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MALE" => Ok(Gender::Male),
            "FEMALE" => Ok(Gender::Female),
            "OTHER" => Ok(Gender::Other),
            "NULL" => Ok(Gender::Null),
            _ => Err(crate::error::ErrorKind::Parsing),
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

impl<'de> Deserialize<'de> for Gender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Gender::from_str(&s).map_err(serde::de::Error::custom)
    }
}
