use crate::common::User;
use crate::error::Error;
use crate::session::UserSession;
use rocket::serde::json::Json;

#[rocket::get("/owner/event_participant_presence_pool?<event_id>")]
pub fn event_participant_presence_pool(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let users = crate::db::event::participant::event_participant_presence_pool(event_id, true)?;
    Ok(Json(users))
}

#[rocket::get("/owner/event_participant_presence_list?<event_id>")]
pub fn event_participant_presence_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let users = crate::db::event::participant::event_participant_presence_list(event_id)?;
    Ok(Json(users))
}

#[rocket::head("/owner/event_participant_presence_add?<event_id>&<user_id>")]
pub fn event_participant_presence_add(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let pool = crate::db::event::participant::event_participant_presence_pool(event_id, true)?;

    if !pool.iter().any(|user| user.id == user_id) {
        return Err(Error::EventPresenceForbidden);
    }

    crate::db::event::participant::event_participant_presence_add(event_id, user_id)?;
    Ok(())
}

#[rocket::head("/owner/event_participant_presence_remove?<event_id>&<user_id>")]
pub fn event_participant_presence_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db::event::participant::event_participant_presence_remove(event_id, user_id)?;
    Ok(())
}
