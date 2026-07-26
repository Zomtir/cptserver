use serde::{Deserialize, Deserializer};

pub use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Occurrence {
    Occurring,
    Canceled,
    Voided,
}

impl Occurrence {
    pub fn as_str(&self) -> &str {
        match self {
            Occurrence::Occurring => "OCCURRING",
            Occurrence::Canceled => "CANCELED",
            Occurrence::Voided => "VOIDED",
        }
    }
}

impl std::fmt::Display for Occurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Occurrence {
    type Err = crate::error::ErrorKind;

    fn from_str<'r>(s: &str) -> Result<Self, Self::Err> {
        match s {
            "OCCURRING" => Ok(Occurrence::Occurring),
            "CANCELED" => Ok(Occurrence::Canceled),
            "VOIDED" => Ok(Occurrence::Voided),
            _ => Err(crate::error::ErrorKind::Parsing),
        }
    }
}

impl core::convert::From<Occurrence> for mysql_common::Value {
    fn from(v: Occurrence) -> Self {
        mysql_common::Value::Bytes(v.to_string().into_bytes())
    }
}

impl<'de> Deserialize<'de> for Occurrence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Occurrence::from_str(&s).map_err(serde::de::Error::custom)
    }
}
