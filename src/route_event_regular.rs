use crate::common::{Acceptance, Confirmation, Event, Occurrence, WebBool, WebDateTime};
use crate::error::Error;
use crate::session::UserSession;
use rocket::serde::json::Json;

/*
 * ROUTES
 */

#[rocket::get("/regular/event_list?<begin>&<end>&<location_id>&<occurrence>&<acceptance>&<course_true>&<course_id>")]
pub fn event_list(
    _session: UserSession,
    begin: Option<WebDateTime>,
    end: Option<WebDateTime>,
    location_id: Option<u64>,
    occurrence: Option<Occurrence>,
    acceptance: Option<Acceptance>,
    course_true: Option<WebBool>,
    course_id: Option<u64>,
) -> Result<Json<Vec<Event>>, Error> {
    let events = crate::db::event::event_list(
        begin.map(|dt| dt.to_naive()),
        end.map(|dt| dt.to_naive()),
        location_id,
        occurrence,
        acceptance,
        course_true.map(|b| b.to_bool()),
        course_id,
        None,
    )?;
    Ok(Json(events))
}

#[rocket::post("/regular/event_create", format = "application/json", data = "<event>")]
pub fn event_create(session: UserSession, mut event: Json<Event>) -> Result<String, Error> {
    crate::common::validate_event_dates(&mut event)?;

    let event_id = crate::db::event::event_create(&event, &Acceptance::Draft, None)?;
    crate::db::event::owner::event_owner_add(event_id, session.user.id)?;
    Ok(event_id.to_string())
}

#[rocket::get("/regular/event_owner_true?<event_id>")]
pub fn event_owner_true(session: UserSession, event_id: u64) -> Result<Json<bool>, Error> {
    let condition = crate::db::event::owner::event_owner_true(event_id, session.user.id)?;
    Ok(Json(condition))
}

#[rocket::get("/regular/event_leader_presence_true?<event_id>")]
pub fn event_leader_presence_true(session: UserSession, event_id: u64) -> Result<Json<bool>, Error> {
    let condition = crate::db::event::leader::event_leader_presence_true(event_id, session.user.id)?;
    Ok(Json(condition))
}

#[rocket::head("/regular/event_leader_presence_add?<event_id>")]
pub fn event_leader_presence_add(session: UserSession, event_id: u64) -> Result<(), Error> {
    let pool = crate::db::event::leader::event_leader_presence_pool(event_id, true)?;

    if pool.iter().any(|user| user.id != session.user.id) {
        return Err(Error::RightEventMissing);
    }

    crate::db::event::leader::event_leader_presence_add(event_id, session.user.id)?;
    Ok(())
}

#[rocket::head("/regular/event_leader_presence_remove?<event_id>")]
pub fn event_leader_presence_remove(session: UserSession, event_id: u64) -> Result<(), Error> {
    crate::db::event::leader::event_leader_presence_remove(event_id, session.user.id)?;
    Ok(())
}

#[rocket::get("/regular/event_supporter_presence_true?<event_id>")]
pub fn event_supporter_presence_true(session: UserSession, event_id: u64) -> Result<Json<bool>, Error> {
    let condition = crate::db::event::supporter::event_supporter_presence_true(event_id, session.user.id)?;
    Ok(Json(condition))
}

#[rocket::head("/regular/event_supporter_presence_add?<event_id>")]
pub fn event_supporter_presence_add(session: UserSession, event_id: u64) -> Result<(), Error> {
    let pool = crate::db::event::supporter::event_supporter_presence_pool(event_id, true)?;

    if pool.iter().any(|user| user.id != session.user.id) {
        return Err(Error::RightEventMissing);
    }

    crate::db::event::supporter::event_supporter_presence_add(event_id, session.user.id)?;
    Ok(())
}

#[rocket::head("/regular/event_supporter_presence_remove?<event_id>")]
pub fn event_supporter_presence_remove(session: UserSession, event_id: u64) -> Result<(), Error> {
    crate::db::event::supporter::event_supporter_presence_remove(event_id, session.user.id)?;
    Ok(())
}

#[rocket::get("/regular/event_participant_presence_true?<event_id>")]
pub fn event_participant_presence_true(session: UserSession, event_id: u64) -> Result<Json<bool>, Error> {
    let condition = crate::db::event::participant::event_participant_presence_true(event_id, session.user.id)?;
    Ok(Json(condition))
}

#[rocket::head("/regular/event_participant_presence_add?<event_id>")]
pub fn event_participant_presence_add(session: UserSession, event_id: u64) -> Result<(), Error> {
    let pool = crate::db::event::participant::event_participant_presence_pool(event_id, true)?;

    if pool.iter().any(|user| user.id != session.user.id) {
        return Err(Error::RightEventMissing);
    }

    crate::db::event::participant::event_participant_presence_add(event_id, session.user.id)?;
    Ok(())
}

#[rocket::head("/regular/event_participant_presence_remove?<event_id>")]
pub fn event_participant_presence_remove(session: UserSession, event_id: u64) -> Result<(), Error> {
    crate::db::event::participant::event_participant_presence_remove(event_id, session.user.id)?;
    Ok(())
}

#[rocket::get("/regular/event_bookmark_true?<event_id>")]
pub fn event_bookmark_true(session: UserSession, event_id: u64) -> Result<Json<bool>, Error> {
    // TODO check if you can participate

    let bookmark = crate::db::event::event_bookmark_true(event_id, session.user.id)?;
    Ok(Json(bookmark))
}

#[rocket::head("/regular/event_bookmark_edit?<event_id>&<bookmark>")]
pub fn event_bookmark_edit(session: UserSession, event_id: u64, bookmark: bool) -> Result<(), Error> {
    // TODO check if you can participate

    match bookmark {
        true => crate::db::event::event_bookmark_add(event_id, session.user.id)?,
        false => crate::db::event::event_bookmark_remove(event_id, session.user.id)?,
    }
    Ok(())
}

#[rocket::get("/regular/event_leader_registration_info?<event_id>")]
pub fn event_leader_registration_info(session: UserSession, event_id: u64) -> Result<String, Error> {
    // TODO check if you can lead (requirement)

    let status = crate::db::event::leader::event_leader_registration_info(event_id, session.user.id)?;
    Ok(status.to_str().to_string())
}

#[rocket::head("/regular/event_leader_registration_edit?<event_id>&<status>")]
pub fn event_leader_registration_edit(session: UserSession, event_id: u64, status: Confirmation) -> Result<(), Error> {
    // TODO check if you can lead (requirement)

    match status {
        Confirmation::Null => crate::db::event::leader::event_leader_registration_remove(event_id, session.user.id)?,
        _ => crate::db::event::leader::event_leader_registration_edit(event_id, session.user.id, status)?,
    }
    Ok(())
}

#[rocket::get("/regular/event_supporter_registration_info?<event_id>")]
pub fn event_supporter_registration_info(session: UserSession, event_id: u64) -> Result<String, Error> {
    // TODO check if you can support (requirement)

    let status = crate::db::event::supporter::event_supporter_registration_info(event_id, session.user.id)?;
    Ok(status.to_str().to_string())
}

#[rocket::head("/regular/event_supporter_registration_edit?<event_id>&<status>")]
pub fn event_supporter_registration_edit(
    session: UserSession,
    event_id: u64,
    status: Confirmation,
) -> Result<(), Error> {
    // TODO check if you can support (requirement)

    match status {
        Confirmation::Null => {
            crate::db::event::supporter::event_supporter_registration_remove(event_id, session.user.id)?
        }
        _ => crate::db::event::supporter::event_supporter_registration_edit(event_id, session.user.id, status)?,
    }
    Ok(())
}

#[rocket::get("/regular/event_participant_registration_info?<event_id>")]
pub fn event_participant_registration_info(session: UserSession, event_id: u64) -> Result<String, Error> {
    // TODO check if you can register (requirement)

    let status = crate::db::event::participant::event_participant_registration_info(event_id, session.user.id)?;
    Ok(status.to_str().to_string())
}

#[rocket::head("/regular/event_participant_registration_edit?<event_id>&<status>")]
pub fn event_participant_registration_edit(
    session: UserSession,
    event_id: u64,
    status: Confirmation,
) -> Result<(), Error> {
    // TODO check if you can register (requirement)

    match status {
        Confirmation::Null => {
            crate::db::event::participant::event_participant_registration_remove(event_id, session.user.id)?
        }
        _ => crate::db::event::participant::event_participant_registration_edit(event_id, session.user.id, status)?,
    }
    Ok(())
}
