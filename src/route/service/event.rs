use rocket::serde::json::Json;

use crate::common::{Event, User};
use crate::error::Error;
use crate::session::EventSession;

#[rocket::get("/service/event_info")]
pub fn event_info(session: EventSession) -> Result<Json<Event>, Error> {
    Ok(Json(crate::db::event::event_info(session.event_id)?))
}

#[rocket::post("/service/event_note_edit", format = "text/plain", data = "<note>")]
pub fn event_note_edit(session: EventSession, note: String) -> Result<(), Error> {
    crate::db::event::event_note_edit(session.event_id, &note)?;
    Ok(())
}

#[rocket::get("/service/event_leader_presence_pool")]
pub fn event_leader_presence_pool(session: EventSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db::event::leader::event_leader_presence_pool(session.event_id, true)?;
    Ok(Json(users))
}

#[rocket::get("/service/event_leader_presence_list")]
pub fn event_leader_presence_list(session: EventSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db::event::leader::event_leader_presence_list(session.event_id)?;
    Ok(Json(users))
}

#[rocket::head("/service/event_leader_presence_add?<user_id>")]
pub fn event_leader_presence_add(user_id: u64, session: EventSession) -> Result<(), Error> {
    let pool = crate::db::event::leader::event_leader_presence_pool(session.event_id, true)?;

    if !pool.iter().any(|user| user.id == user_id) {
        return Err(Error::EventPresenceForbidden);
    }
    crate::db::event::leader::event_leader_presence_add(session.event_id, user_id)
}

#[rocket::head("/service/event_leader_presence_remove?<user_id>")]
pub fn event_leader_presence_remove(user_id: u64, session: EventSession) -> Result<(), Error> {
    crate::db::event::leader::event_leader_presence_remove(session.event_id, user_id)
}

#[rocket::get("/service/event_supporter_presence_pool")]
pub fn event_supporter_presence_pool(session: EventSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db::event::supporter::event_supporter_presence_pool(session.event_id, true)?;
    Ok(Json(users))
}

#[rocket::get("/service/event_supporter_presence_list")]
pub fn event_supporter_presence_list(session: EventSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db::event::supporter::event_supporter_presence_list(session.event_id)?;
    Ok(Json(users))
}

#[rocket::head("/service/event_supporter_presence_add?<user_id>")]
pub fn event_supporter_presence_add(user_id: u64, session: EventSession) -> Result<(), Error> {
    let pool = crate::db::event::supporter::event_supporter_presence_pool(session.event_id, true)?;

    if !pool.iter().any(|user| user.id == user_id) {
        return Err(Error::EventPresenceForbidden);
    }
    crate::db::event::supporter::event_supporter_presence_add(session.event_id, user_id)
}

#[rocket::head("/service/event_supporter_presence_remove?<user_id>")]
pub fn event_supporter_presence_remove(user_id: u64, session: EventSession) -> Result<(), Error> {
    crate::db::event::supporter::event_supporter_presence_remove(session.event_id, user_id)
}

#[rocket::get("/service/event_participant_presence_pool")]
pub fn event_participant_presence_pool(session: EventSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db::event::participant::event_participant_presence_pool(session.event_id, true)?;
    Ok(Json(users))
}

#[rocket::get("/service/event_participant_presence_list")]
pub fn event_participant_presence_list(session: EventSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db::event::participant::event_participant_presence_list(session.event_id)?;
    Ok(Json(users))
}

#[rocket::head("/service/event_participant_presence_add?<user_id>")]
pub fn event_participant_presence_add(user_id: u64, session: EventSession) -> Result<(), Error> {
    let pool = crate::db::event::participant::event_participant_presence_pool(session.event_id, true)?;

    if !pool.iter().any(|user| user.id == user_id) {
        return Err(Error::EventPresenceForbidden);
    }
    crate::db::event::participant::event_participant_presence_add(session.event_id, user_id)
}

#[rocket::head("/service/event_participant_presence_remove?<user_id>")]
pub fn event_participant_presence_remove(user_id: u64, session: EventSession) -> Result<(), Error> {
    crate::db::event::participant::event_participant_presence_remove(session.event_id, user_id)
}
