use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::{Acceptance, Course, Event, Occurrence, User, WebDateTime};
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;

pub async fn owner_list(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;
    Ok(Json(crate::db::event::owner::event_owner_list(conn, event_id)?))
}

pub async fn owner_add(
    State(state): State<AppState>,
    session: UserSession,
    Path((event_id, user_id)): Path<(u64, u64)>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;
    crate::db::event::owner::event_owner_add(conn, event_id, user_id)?;
    Ok(())
}

pub async fn owner_remove(
    State(state): State<AppState>,
    session: UserSession,
    Path((event_id, user_id)): Path<(u64, u64)>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;
    if user_id == session.user.id {
        return Err(Error::new(ErrorKind::Protected, "Cannot remove self from event owners"));
    }
    crate::db::event::owner::event_owner_remove(conn, event_id, user_id)?;
    Ok(())
}
