/*
 * Time and chronology related stuff
 */

use chrono::{NaiveDateTime, DateTime, Utc, Duration, DurationRound};

pub mod date_format {
    use chrono::{NaiveDate};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(
        date: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error>
    where D: Deserializer<'de>, {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub mod datetime_format {
    use chrono::{NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M";

    pub fn serialize<S>(
        datetime: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where S: Serializer, {
        let s = format!("{}", datetime.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error>
    where D: Deserializer<'de>, {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

pub trait NaiveDurationRound : Sized {
    fn duration_round(self, duration: Duration) -> Result<Self, <DateTime<Utc> as DurationRound>::Err>;
}

impl NaiveDurationRound for NaiveDateTime {
    fn duration_round(self, duration: Duration) -> Result<Self, <DateTime<Utc> as DurationRound>::Err> {
        match DateTime::<Utc>::from_utc(self, Utc).duration_round(duration) {
            Err(e) => Err(e),
            Ok(dt) => Ok(dt.naive_utc()),
        }
    }
}
