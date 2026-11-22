use axum::extract::{Path, State};
use axum::Json;

use crate::common::User;
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;

pub async fn owner_list(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_read).or(crate::permission::require_event_owner(
        conn,
        event_id,
        session.user.id,
    ))?;
    Ok(Json(crate::db::event::owner::event_owner_list(conn, event_id)?))
}

pub async fn owner_true(
    State(state): State<AppState>,
    session: UserSession,
    Path((event_id, user_id)): Path<(u64, u64)>,
) -> Result<Json<bool>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_read)
        .or(crate::permission::require_event_owner(conn, event_id, session.user.id))
        .or(crate::permission::is_self(&session, Some(user_id)))?;

    let condition = crate::db::event::owner::event_owner_true(conn, event_id, user_id)?;
    Ok(Json(condition))
}

pub async fn owner_add(
    State(state): State<AppState>,
    session: UserSession,
    Path((event_id, user_id)): Path<(u64, u64)>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_write).or(crate::permission::require_event_owner(
        conn,
        event_id,
        session.user.id,
    ))?;
    crate::db::event::owner::event_owner_add(conn, event_id, user_id)?;
    Ok(())
}

pub async fn owner_remove(
    State(state): State<AppState>,
    session: UserSession,
    Path((event_id, user_id)): Path<(u64, u64)>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_write).or(crate::permission::require_event_owner(
        conn,
        event_id,
        session.user.id,
    ))?;
    if !session.right.right_event_write && user_id == session.user.id {
        return Err(Error::new(ErrorKind::Protected, "Cannot remove self from event owners"));
    }
    crate::db::event::owner::event_owner_remove(conn, event_id, user_id)?;
    Ok(())
}
