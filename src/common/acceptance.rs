use serde::{Deserialize, Deserializer};

pub use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Acceptance {
    Draft,
    Pending,
    Accepted,
    Rejected,
}

impl Acceptance {
    pub fn as_str(&self) -> &str {
        match self {
            Acceptance::Draft => "DRAFT",
            Acceptance::Pending => "PENDING",
            Acceptance::Accepted => "ACCEPTED",
            Acceptance::Rejected => "REJECTED",
        }
    }
}

impl std::fmt::Display for Acceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Acceptance {
    type Err = crate::error::ErrorKind;

    fn from_str<'r>(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DRAFT" => Ok(Acceptance::Draft),
            "PENDING" => Ok(Acceptance::Pending),
            "ACCEPTED" => Ok(Acceptance::Accepted),
            "REJECTED" => Ok(Acceptance::Rejected),
            _ => Err(crate::error::ErrorKind::Parsing),
        }
    }
}

impl core::convert::From<Acceptance> for mysql_common::Value {
    fn from(a: Acceptance) -> Self {
        mysql_common::Value::Bytes(a.to_string().into_bytes())
    }
}

impl<'de> Deserialize<'de> for Acceptance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Acceptance::from_str(&s).map_err(serde::de::Error::custom)
    }
}
