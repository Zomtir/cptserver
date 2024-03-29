use rocket::serde::json::Json;

use crate::common::{Event, User};
use crate::error::Error;
use crate::session::EventSession;

/*
 * ROUTES
 */

#[rocket::get("/service/event_info")]
pub fn event_info(session: EventSession) -> Result<Json<Event>, Error> {
    Ok(Json(crate::db_event::event_info(session.event_id)?))
}

#[rocket::post("/service/event_note_edit", format = "text/plain", data = "<note>")]
pub fn event_note_edit(session: EventSession, note: String) -> Result<(), Error> {
    crate::db_event::event_note_edit(session.event_id, &note)?;
    Ok(())
}

#[rocket::get("/service/event_participant_pool")]
pub fn event_participant_pool(session: EventSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db_event::event_participant_pool(session.event_id)?;
    Ok(Json(users))
}

#[rocket::get("/service/event_participant_list")]
pub fn event_participant_list(session: EventSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db_event::event_participant_list(session.event_id)?;
    Ok(Json(users))
}

#[rocket::head("/service/event_participant_add?<user_id>")]
pub fn event_participant_add(user_id: u64, session: EventSession) -> Result<(), Error> {
    crate::db_event::event_participant_add(session.event_id, user_id)
}

#[rocket::head("/service/event_participant_remove?<user_id>")]
pub fn event_participant_remove(user_id: u64, session: EventSession) -> Result<(), Error> {
    crate::db_event::event_participant_remove(session.event_id, user_id)
}

#[rocket::get("/service/event_owner_pool")]
pub fn event_owner_pool(session: EventSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db_event::event_owner_pool(session.event_id)?;
    Ok(Json(users))
}

#[rocket::get("/service/event_owner_list")]
pub fn event_owner_list(session: EventSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db_event::event_owner_list(session.event_id)?;
    Ok(Json(users))
}

#[rocket::head("/service/event_owner_add?<user_id>")]
pub fn event_owner_add(user_id: u64, session: EventSession) -> Result<(), Error> {
    crate::db_event::event_owner_add(session.event_id, user_id)
}

#[rocket::head("/service/event_owner_remove?<user_id>")]
pub fn event_owner_remove(user_id: u64, session: EventSession) -> Result<(), Error> {
    crate::db_event::event_owner_remove(session.event_id, user_id)
}
