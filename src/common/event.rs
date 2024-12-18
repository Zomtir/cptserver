use crate::common::Location;
use crate::error::Error;
use chrono::DurationRound;
use serde::{Deserialize, Serialize};

/*
 * STRUCTS
 */

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: u64,
    pub key: String,
    pub title: String,
    pub begin: chrono::NaiveDateTime,
    pub end: chrono::NaiveDateTime,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pwd: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrutable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub course_id: Option<u32>,
}

impl Event {
    pub fn from_info(
        id: u64,
        key: String,
        title: String,
        begin: chrono::NaiveDateTime,
        end: chrono::NaiveDateTime,
        location: Option<Location>,
    ) -> Event {
        Event {
            id,
            key,
            pwd: None,
            title,
            begin,
            end,
            location: location,
            note: None,
            occurrence: None,
            acceptance: None,
            public: None,
            scrutable: None,
            course_id: None,
        }
    }

    pub fn sqlmap() -> impl Fn(
        (
            u64,
            String,
            String,
            chrono::NaiveDateTime,
            chrono::NaiveDateTime,
            Option<u32>,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    ) -> Event {
        |(event_id, event_key, title, begin, end, location_id, location_key, location_name, location_description)| {
            Event::from_info(
                event_id,
                event_key,
                title,
                begin,
                end,
                location_id.map(|id| Location {
                    id,
                    key: location_key.unwrap(),
                    name: location_name.unwrap(),
                    description: location_description.unwrap(),
                }),
            )
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
    event.begin + crate::config::EVENT_OCCURRENCE_DURATION_MIN() < event.end
        || event.begin + crate::config::EVENT_OCCURRENCE_DURATION_MAX() > event.end
}

pub fn validate_event_dates(event: &mut Event) -> Result<(), Error> {
    event.begin = event.begin.duration_round(crate::config::EVENT_OCCURRENCE_SNAP())?;
    event.end = event.end.duration_round(crate::config::EVENT_OCCURRENCE_SNAP())?;

    let earliest_end = event.begin + crate::config::EVENT_OCCURRENCE_DURATION_MIN();

    if earliest_end > event.end {
        event.end = earliest_end;
    }

    let latest_end = event.begin + crate::config::EVENT_OCCURRENCE_DURATION_MAX();

    if latest_end < event.end {
        event.end = latest_end;
    }

    Ok(())
}
