use serde::{Deserialize, Deserializer};
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
            _ => Err(crate::error::Error::new(
                crate::error::ErrorKind::Parsing,
                "Invalid confirmation value",
            )),
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

impl<'de> Deserialize<'de> for Confirmation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Confirmation::from_str(&s).map_err(serde::de::Error::custom)
    }
}
