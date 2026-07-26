use crate::common::User;
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use axum::Json;

pub fn event_owner_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    let users = crate::db::event::owner::event_owner_list(conn, event_id)?;
    Ok(Json(users))
}

pub fn event_owner_add(session: UserSession, event_id: u64, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    crate::db::event::owner::event_owner_add(conn, event_id, user_id)?;
    Ok(())
}

pub fn event_owner_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    if user_id == session.user.id {
        return Err(Error::new(ErrorKind::Protected, "Cannot remove self from event owners"));
    };

    crate::db::event::owner::event_owner_remove(conn, event_id, user_id)?;
    Ok(())
}
