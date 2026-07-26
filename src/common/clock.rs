use crate::error::{Error, ErrorKind};
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

/*
 * Time and chronology related stuff
 */

/// WebDate
pub struct WebDate(pub chrono::NaiveDate);

impl WebDate {
    pub fn to_naive(&self) -> chrono::NaiveDate {
        self.0
    }
}

impl std::str::FromStr for WebDate {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|_| Error::new(ErrorKind::Invalid, "Invalid date"))?;

        Ok(WebDate(date))
    }
}

impl<'de> Deserialize<'de> for WebDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        WebDate::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// WebDateTime
pub struct WebDateTime(pub chrono::NaiveDateTime);

impl WebDateTime {
    pub fn to_naive(&self) -> chrono::NaiveDateTime {
        self.0
    }
}

impl std::str::FromStr for WebDateTime {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let datetime = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d-%H-%M")
            .map_err(|_| Error::new(ErrorKind::Invalid, "Invalid datetime"))?;

        Ok(WebDateTime(datetime))
    }
}

impl<'de> Deserialize<'de> for WebDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        WebDateTime::from_str(&s).map_err(serde::de::Error::custom)
    }
}
