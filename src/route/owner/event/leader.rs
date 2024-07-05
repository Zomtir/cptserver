use crate::common::User;
use crate::error::Error;
use crate::session::UserSession;
use rocket::serde::json::Json;

#[rocket::get("/owner/event_leader_registration_list?<event_id>")]
pub fn registration_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    let users = crate::db::event::leader::event_leader_registration_list(event_id)?;
    Ok(Json(users))
}

#[rocket::get("/owner/event_leader_filter_list?<event_id>")]
pub fn filter_list(session: UserSession, event_id: u64) -> Result<Json<Vec<(User, bool)>>, Error> {
    if !crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    let filters = crate::db::event::leader::event_leader_filter_list(event_id)?;
    Ok(Json(filters))
}

#[rocket::head("/owner/event_leader_filter_edit?<event_id>&<user_id>&<access>")]
pub fn filter_edit(session: UserSession, event_id: u64, user_id: u64, access: bool) -> Result<(), Error> {
    if !crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    crate::db::event::leader::event_leader_filter_edit(event_id, user_id, access)?;
    Ok(())
}

#[rocket::head("/owner/event_leader_filter_remove?<event_id>&<user_id>")]
pub fn filter_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    if !crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    crate::db::event::leader::event_leader_filter_remove(event_id, user_id)?;
    Ok(())
}

#[rocket::get("/owner/event_leader_presence_pool?<event_id>")]
pub fn presence_pool(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    let users = crate::db::event::leader::event_leader_presence_pool(event_id, true)?;
    Ok(Json(users))
}

#[rocket::get("/owner/event_leader_presence_list?<event_id>")]
pub fn presence_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    let users = crate::db::event::leader::event_leader_presence_list(event_id)?;
    Ok(Json(users))
}

#[rocket::head("/owner/event_leader_presence_add?<event_id>&<user_id>")]
pub fn presence_add(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    if !crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    let pool = crate::db::event::leader::event_leader_presence_pool(event_id, true)?;

    if !pool.iter().any(|user| user.id == user_id) {
        return Err(Error::EventPresenceForbidden);
    }

    crate::db::event::leader::event_leader_presence_add(event_id, user_id)?;
    Ok(())
}

#[rocket::head("/owner/event_leader_presence_remove?<event_id>&<user_id>")]
pub fn presence_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    if !crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    crate::db::event::leader::event_leader_presence_remove(event_id, user_id)?;
    Ok(())
}
