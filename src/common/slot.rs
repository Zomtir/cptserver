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
pub struct Slot {
    pub id: i64,
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
pub enum SlotStatus {
    Draft,
    Pending,
    Occurring,
    Rejected,
    Canceled,
}

impl SlotStatus {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "DRAFT" => Some(SlotStatus::Draft),
            "PENDING" => Some(SlotStatus::Pending),
            "OCCURRING" => Some(SlotStatus::Occurring),
            "REJECTED" => Some(SlotStatus::Rejected),
            "CANCELED" => Some(SlotStatus::Canceled),
            _ => None,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            SlotStatus::Draft => "DRAFT",
            SlotStatus::Pending => "PENDING",
            SlotStatus::Occurring => "OCCURRING",
            SlotStatus::Rejected => "REJECTED",
            SlotStatus::Canceled => "CANCELED",
        }
    }
}

impl core::convert::From<SlotStatus> for mysql_common::Value {
    fn from(s: SlotStatus) -> Self {
        mysql_common::Value::Bytes(s.to_str().to_string().into_bytes())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for SlotStatus {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match SlotStatus::from_str(field.value) {
            None => Err(form::Errors::default()),
            Some(slot_status) => Ok(slot_status),
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        let web_string: String = match field.data.open(200.bytes()).into_string().await {
            Err(..) => return Err(form::Errors::default()),
            Ok(string) => string.into_inner(),
        };

        match SlotStatus::from_str(&web_string) {
            None => return Err(form::Errors::default()),
            Some(slot_status) => return Ok(slot_status),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Location {
    pub id: u32,
    pub key: String,
    pub title: String,
}

/*
 * METHODS
 */

pub fn validate_clear_password(password: String) -> Result<String, Error> {
    if password.len() < 6 || password.len() > 50 {
        return Err(Error::SlotPasswordInvalid);
    };

    Ok(password.to_string())
}

pub fn is_slot_valid(slot: &Slot) -> bool {
    slot.begin + crate::config::CONFIG_SLOT_WINDOW_MINIMUM() < slot.end
}

pub fn validate_slot_dates(slot: &mut Slot) -> Result<(), Error> {
    slot.begin = slot.begin.duration_round(crate::config::CONFIG_SLOT_WINDOW_SNAP())?;

    slot.end = slot.end.duration_round(crate::config::CONFIG_SLOT_WINDOW_SNAP())?;

    let earliest_end = slot.begin + crate::config::CONFIG_SLOT_WINDOW_MINIMUM();

    if earliest_end > slot.end {
        slot.end = earliest_end;
    }

    Ok(())
}
