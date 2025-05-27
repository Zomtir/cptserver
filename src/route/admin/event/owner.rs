use rocket::serde::json::Json;

use crate::common::User;
use crate::error::{ErrorKind, Result};
use crate::session::UserSession;

#[rocket::get("/admin/event_owner_list?<event_id>")]
pub fn owner_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_read {
        return Err(ErrorKind::RightEventMissing);
    };

    let filters = crate::db::event::owner::event_owner_list(conn, event_id)?;
    Ok(Json(filters))
}

#[rocket::head("/admin/event_owner_add?<event_id>&<user_id>")]
pub fn owner_add(session: UserSession, event_id: u64, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_write {
        return Err(ErrorKind::RightEventMissing);
    };

    crate::db::event::owner::event_owner_add(conn, event_id, user_id)?;
    Ok(())
}

#[rocket::head("/admin/event_owner_remove?<event_id>&<user_id>")]
pub fn owner_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_write {
        return Err(ErrorKind::RightEventMissing);
    };

    crate::db::event::owner::event_owner_remove(conn, event_id, user_id)?;
    Ok(())
}
