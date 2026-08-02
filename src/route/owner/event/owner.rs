use crate::common::User;
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;
use axum::extract::State;
use axum::Json;

pub fn event_owner_list(State(state): State<AppState>, session: UserSession, event_id: u64) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    let users = crate::db::event::owner::event_owner_list(conn, event_id)?;
    Ok(Json(users))
}

pub fn event_owner_add(State(state): State<AppState>, session: UserSession, event_id: u64, user_id: u64) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    crate::db::event::owner::event_owner_add(conn, event_id, user_id)?;
    Ok(())
}

pub fn event_owner_remove(
    State(state): State<AppState>,
    session: UserSession,
    event_id: u64,
    user_id: u64,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    if user_id == session.user.id {
        return Err(Error::new(ErrorKind::Protected, "Cannot remove self from event owners"));
    };

    crate::db::event::owner::event_owner_remove(conn, event_id, user_id)?;
    Ok(())
}
