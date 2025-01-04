use rocket::serde::json::Json;

use crate::common::User;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/event_leader_registration_list?<event_id>")]
pub fn registration_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_read {
        return Err(Error::RightEventMissing);
    };

    let users = crate::db::event::leader::event_leader_registration_list(conn, event_id)?;
    Ok(Json(users))
}

#[rocket::get("/admin/event_leader_filter_list?<event_id>")]
pub fn filter_list(session: UserSession, event_id: u64) -> Result<Json<Vec<(User, bool)>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_read {
        return Err(Error::RightEventMissing);
    };

    let filters = crate::db::event::leader::event_leader_filter_list(conn, event_id)?;
    Ok(Json(filters))
}

#[rocket::head("/admin/event_leader_filter_edit?<event_id>&<user_id>&<access>")]
pub fn filter_edit(session: UserSession, event_id: u64, user_id: u64, access: bool) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_write {
        return Err(Error::RightEventMissing);
    };

    crate::db::event::leader::event_leader_filter_edit(conn, event_id, user_id, access)?;
    Ok(())
}

#[rocket::head("/admin/event_leader_filter_remove?<event_id>&<user_id>")]
pub fn filter_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_write {
        return Err(Error::RightEventMissing);
    };

    crate::db::event::leader::event_leader_filter_remove(conn, event_id, user_id)?;
    Ok(())
}

#[rocket::get("/admin/event_leader_presence_pool?<event_id>")]
pub fn presence_pool(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_read {
        return Err(Error::RightEventMissing);
    };

    let users = crate::db::event::leader::event_leader_presence_pool(conn, event_id, true)?;
    Ok(Json(users))
}

#[rocket::get("/admin/event_leader_presence_list?<event_id>")]
pub fn presence_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_read {
        return Err(Error::RightEventMissing);
    };

    let users = crate::db::event::leader::event_leader_presence_list(conn, event_id)?;
    Ok(Json(users))
}

#[rocket::head("/admin/event_leader_presence_add?<event_id>&<user_id>")]
pub fn presence_add(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_write {
        return Err(Error::RightEventMissing);
    };

    let pool = crate::db::event::leader::event_leader_presence_pool(conn, event_id, true)?;

    if !pool.iter().any(|user| user.id == user_id) {
        return Err(Error::EventPresenceForbidden);
    }

    crate::db::event::leader::event_leader_presence_add(conn, event_id, user_id)?;
    Ok(())
}

#[rocket::head("/admin/event_leader_presence_remove?<event_id>&<user_id>")]
pub fn presence_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_write {
        return Err(Error::RightEventMissing);
    };

    crate::db::event::leader::event_leader_presence_remove(conn, event_id, user_id)?;
    Ok(())
}
