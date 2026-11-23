use serde::{Deserialize, Deserializer};
pub use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Confirmation {
    Positive,
    Neutral,
    Negative,
}

impl Confirmation {
    pub fn as_str(&self) -> &str {
        match self {
            Confirmation::Positive => "POSITIVE",
            Confirmation::Neutral => "NEUTRAL",
            Confirmation::Negative => "NEGATIVE",
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
            _ => Err(crate::error::Error::new(
                crate::error::ErrorKind::Parsing,
                "Invalid confirmation value",
            )),
        }
    }
}

impl core::convert::From<Confirmation> for mysql_common::Value {
    fn from(s: Confirmation) -> Self {
        mysql_common::Value::Bytes(s.to_string().into_bytes())
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
