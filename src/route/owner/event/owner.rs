use crate::common::User;
use crate::error::Error;
use crate::session::UserSession;
use rocket::serde::json::Json;

#[rocket::get("/owner/event_owner_list?<event_id>")]
pub fn event_owner_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    let users = crate::db::event::owner::event_owner_list(event_id)?;
    Ok(Json(users))
}

#[rocket::head("/owner/event_owner_add?<event_id>&<user_id>")]
pub fn event_owner_add(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    if !crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    crate::db::event::owner::event_owner_add(event_id, user_id)?;
    Ok(())
}

#[rocket::head("/owner/event_owner_remove?<event_id>&<user_id>")]
pub fn event_owner_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    if !crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    if user_id == session.user.id {
        return Err(Error::EventOwnerProtection);
    };

    crate::db::event::owner::event_owner_remove(event_id, user_id)?;
    Ok(())
}
