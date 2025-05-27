use crate::common::Event;
use crate::error::{ErrorKind, Result};
use chrono::DurationRound;

pub fn validate_clear_password(password: String) -> Result<String> {
    if password.len() < 6 || password.len() > 50 {
        return Err(ErrorKind::EventPasswordInvalid);
    };

    Ok(password.to_string())
}

pub fn is_event_valid(event: &Event) -> bool {
    event.begin + crate::config::EVENT_OCCURRENCE_DURATION_MIN() < event.end
        || event.begin + crate::config::EVENT_OCCURRENCE_DURATION_MAX() > event.end
}

pub fn validate_event_dates(event: &mut Event) -> Result<()> {
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

pub fn verify_event_search_window(
    begin: Option<chrono::NaiveDateTime>,
    end: Option<chrono::NaiveDateTime>,
) -> Result<()> {
    // If there is a search window, make sure it is somewhat correct
    if let (Some(begin), Some(end)) = (begin, end) {
        let delta = end.signed_duration_since(begin);

        if delta < crate::config::EVENT_SEARCH_WINDOW_MIN() || delta > crate::config::EVENT_SEARCH_WINDOW_MAX() {
            return Err(ErrorKind::EventSearchLimit);
        }

        if begin < crate::config::EVENT_SEARCH_DATE_MIN() || end > crate::config::EVENT_SEARCH_DATE_MAX() {
            return Err(ErrorKind::EventSearchLimit);
        }
    }
    Ok(())
}
