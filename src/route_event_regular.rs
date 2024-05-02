use crate::common::{AcceptanceStatus, ConfirmationStatus, Event, WebBool, WebDateTime};
use crate::error::Error;
use crate::session::UserSession;
use rocket::serde::json::Json;

/*
 * ROUTES
 */

#[rocket::get("/regular/event_list?<begin>&<end>&<status>&<location_id>&<course_true>&<course_id>")]
pub fn event_list(
    _session: UserSession,
    begin: Option<WebDateTime>,
    end: Option<WebDateTime>,
    location_id: Option<u64>,
    status: Option<AcceptanceStatus>,
    course_true: Option<WebBool>,
    course_id: Option<u64>,
) -> Result<Json<Vec<Event>>, Error> {
    let events = crate::db_event::event_list(
        begin.map(|dt| dt.to_naive()),
        end.map(|dt| dt.to_naive()),
        status,
        location_id,
        course_true.map(|b| b.to_bool()),
        course_id,
        None,
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

#[rocket::get("/regular/event_participant_true?<event_id>")]
pub fn event_participant_true(session: UserSession, event_id: u64) -> Result<Json<bool>, Error> {
    let condition = crate::db_event::event_participant_true(event_id, session.user.id)?;
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

#[rocket::get("/regular/event_bookmark_true?<event_id>")]
pub fn event_bookmark_true(session: UserSession, event_id: u64) -> Result<Json<bool>, Error> {
    // TODO check if you can participate

    let bookmark = crate::db_event::event_bookmark_true(event_id, session.user.id)?;
    Ok(Json(bookmark))
}

#[rocket::head("/regular/event_bookmark_edit?<event_id>&<bookmark>")]
pub fn event_bookmark_edit(session: UserSession, event_id: u64, bookmark: bool) -> Result<(), Error> {
    // TODO check if you can participate

    match bookmark {
        true => crate::db_event::event_bookmark_add(event_id, session.user.id)?,
        false => crate::db_event::event_bookmark_remove(event_id, session.user.id)?,
    }
    Ok(())
}

#[rocket::get("/regular/event_owner_registration_status?<event_id>")]
pub fn event_owner_registration_status(session: UserSession, event_id: u64) -> Result<String, Error> {
    // TODO check if you can own

    let status = crate::db_event::event_owner_registration_status(event_id, session.user.id)?;
    Ok(status.to_str().to_string())
}

#[rocket::head("/regular/event_owner_registration_edit?<event_id>&<status>")]
pub fn event_owner_registration_edit(
    session: UserSession,
    event_id: u64,
    status: ConfirmationStatus,
) -> Result<(), Error> {
    // TODO check if you can own

    match status {
        ConfirmationStatus::Null => crate::db_event::event_owner_registration_remove(event_id, session.user.id)?,
        _ => crate::db_event::event_owner_registration_edit(event_id, session.user.id, status)?,
    }
    Ok(())
}

#[rocket::get("/regular/event_participant_registration_status?<event_id>")]
pub fn event_participant_registration_status(session: UserSession, event_id: u64) -> Result<String, Error> {
    // TODO check if you can participate

    let status = crate::db_event::event_participant_registration_status(event_id, session.user.id)?;
    Ok(status.to_str().to_string())
}

#[rocket::head("/regular/event_participant_registration_edit?<event_id>&<status>")]
pub fn event_participant_registration_edit(
    session: UserSession,
    event_id: u64,
    status: ConfirmationStatus,
) -> Result<(), Error> {
    // TODO check if you can participate

    match status {
        ConfirmationStatus::Null => crate::db_event::event_participant_registration_remove(event_id, session.user.id)?,
        _ => crate::db_event::event_participant_registration_edit(event_id, session.user.id, status)?,
    }
    Ok(())
}
