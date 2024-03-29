use crate::common::{Event, EventStatus, WebDateTime};
use crate::error::Error;
use crate::session::UserSession;
use rocket::serde::json::Json;

/*
 * ROUTES
 */

#[rocket::get("/regular/event_list?<begin>&<end>&<status>&<location_id>&<course_true>&<course_id>")]
pub fn event_list(
    session: UserSession,
    begin: Option<WebDateTime>,
    end: Option<WebDateTime>,
    location_id: Option<u64>,
    status: Option<EventStatus>,
    course_true: Option<bool>,
    course_id: Option<u64>,
) -> Result<Json<Vec<Event>>, Error> {
    let events = crate::db_event::event_list(
        begin.map(|dt| dt.to_naive()),
        end.map(|dt| dt.to_naive()),
        status,
        location_id,
        course_true,
        course_id,
        Some(session.user.id),
    )?;
    Ok(Json(events))
}

#[rocket::post("/regular/event_create", format = "application/json", data = "<event>")]
pub fn event_create(session: UserSession, mut event: Json<Event>) -> Result<String, Error> {
    crate::common::validate_event_dates(&mut event)?;

    let event_id = crate::db_event::event_create(&event, "DRAFT", None)?;
    crate::db_event::event_owner_add(event_id, session.user.id)?;
    Ok(event_id.to_string())
}

#[rocket::get("/regular/event_owner_true?<event_id>")]
pub fn event_owner_true(session: UserSession, event_id: u64) -> Result<Json<bool>, Error> {
    let condition = crate::db_event::event_owner_true(event_id, session.user.id)?;
    Ok(Json(condition))
}

#[rocket::head("/regular/event_participant_add?<event_id>")]
pub fn event_participant_add(session: UserSession, event_id: u64) -> Result<(), Error> {
    // TODO check if you can participate

    crate::db_event::event_participant_add(event_id, session.user.id)?;
    Ok(())
}

#[rocket::head("/regular/event_participant_remove?<event_id>")]
pub fn event_participant_remove(session: UserSession, event_id: u64) -> Result<(), Error> {
    // TODO check if you can participate

    crate::db_event::event_participant_remove(event_id, session.user.id)?;
    Ok(())
}
