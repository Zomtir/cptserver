use crate::common::Location;
use crate::error::Error;
use chrono::DurationRound;
use rocket::{
    data::ToByteUnit,
    form::{self, DataField, FromFormField, ValueField},
};
use serde::{Deserialize, Serialize};

/*
 * STRUCTS
 */

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: u64,
    pub key: String,
    pub pwd: Option<String>,
    pub title: String,
    pub location: Location,
    pub begin: chrono::NaiveDateTime,
    pub end: chrono::NaiveDateTime,
    pub status: String,
    pub public: bool,
    pub scrutable: bool,
    pub note: String,
    pub course_id: Option<u32>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EventStatus {
    Draft,
    Pending,
    Occurring,
    Rejected,
    Canceled,
}

impl EventStatus {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "DRAFT" => Some(EventStatus::Draft),
            "PENDING" => Some(EventStatus::Pending),
            "OCCURRING" => Some(EventStatus::Occurring),
            "REJECTED" => Some(EventStatus::Rejected),
            "CANCELED" => Some(EventStatus::Canceled),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            EventStatus::Draft => "DRAFT",
            EventStatus::Pending => "PENDING",
            EventStatus::Occurring => "OCCURRING",
            EventStatus::Rejected => "REJECTED",
            EventStatus::Canceled => "CANCELED",
        }
    }
}

impl core::convert::From<EventStatus> for mysql_common::Value {
    fn from(s: EventStatus) -> Self {
        mysql_common::Value::Bytes(s.to_str().to_string().into_bytes())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for EventStatus {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match EventStatus::from_str(field.value) {
            None => Err(form::Errors::default()),
            Some(event_status) => Ok(event_status),
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let web_string: String = match field.data.open(200.bytes()).into_string().await {
            Err(..) => return Err(form::Errors::default()),
            Ok(string) => string.into_inner(),
        };

        match EventStatus::from_str(&web_string) {
            None => return Err(form::Errors::default()),
            Some(event_status) => return Ok(event_status),
        }
    }
}

/*
 * METHODS
 */

pub fn validate_clear_password(password: String) -> Result<String, Error> {
    if password.len() < 6 || password.len() > 50 {
        return Err(Error::EventPasswordInvalid);
    };

    Ok(password.to_string())
}

pub fn is_event_valid(event: &Event) -> bool {
    event.begin + crate::config::CONFIG_SLOT_WINDOW_MINIMUM() < event.end
}

pub fn validate_event_dates(event: &mut Event) -> Result<(), Error> {
    event.begin = event.begin.duration_round(crate::config::CONFIG_SLOT_WINDOW_SNAP())?;

    event.end = event.end.duration_round(crate::config::CONFIG_SLOT_WINDOW_SNAP())?;

    let earliest_end = event.begin + crate::config::CONFIG_SLOT_WINDOW_MINIMUM();

    if earliest_end > event.end {
        event.end = earliest_end;
    }

    Ok(())
}
