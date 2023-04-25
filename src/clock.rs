/*
 * Time and chronology related stuff
 */

use chrono::{DateTime, Duration, DurationRound, Utc};
use rocket::{
    data::ToByteUnit,
    form::{self, DataField, FromFormField, ValueField},
};

use crate::error::Error;
pub struct WebDate(chrono::NaiveDate);
pub struct WebDateTime(chrono::NaiveDateTime);

pub struct SqlDate(chrono::NaiveDate);
pub struct SqlDateTime(chrono::NaiveDateTime);

pub mod date_format {
    use serde::{self, Deserialize, Deserializer, Serializer};

    const _FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &chrono::NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(_FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<chrono::NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        chrono::NaiveDate::parse_from_str(&s, _FORMAT).map_err(serde::de::Error::custom)
    }
}

pub mod datetime_format {
    use serde::{self, Deserialize, Deserializer, Serializer};

    const _FORMAT: &'static str = "%Y-%m-%d %H:%M";

    pub fn serialize<S>(datetime: &chrono::NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", datetime.format(_FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<chrono::NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        chrono::NaiveDateTime::parse_from_str(&s, _FORMAT).map_err(serde::de::Error::custom)
    }
}

pub fn duration_round(naive_datetime: chrono::NaiveDateTime, duration: Duration) -> Result<chrono::NaiveDateTime, Error>
{
    Ok(DateTime::<Utc>::from_utc(naive_datetime, Utc).duration_round(duration)?.naive_utc())
}

impl WebDate {
    fn parse_from_str(s: &str) -> Option<WebDate> {
        match chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            Err(..) => None,
            Ok(naive_date) => Some(WebDate(naive_date)),
        }
    }

    pub fn to_naive(self) -> chrono::NaiveDate { self.0 }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for WebDate {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match WebDate::parse_from_str(field.value) {
            None => return Err(form::Errors::default()),
            Some(web_date) => return Ok(web_date),
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let web_string : String = match field.data.open(200.bytes()).into_string().await {
            Err(..) => return Err(form::Errors::default()),
            Ok(string) => string.into_inner(),
        };

        match WebDate::parse_from_str(&web_string) {
            None => return Err(form::Errors::default()),
            Some(web_date) => return Ok(web_date),
        }
    }
}
