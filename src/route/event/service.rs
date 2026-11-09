use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::User;
use crate::error::{Error, ErrorKind, Result};
use crate::session::EventSession;
use crate::AppState;

#[derive(Deserialize)]
pub struct EventServiceRoleQuery {
    pub role: String,
}

#[derive(Deserialize)]
pub struct EventServicePresenceQuery {
    pub user_id: u64,
    pub role: String,
}

pub async fn event_info(State(state): State<AppState>, session: EventSession) -> Result<Json<crate::common::Event>> {
    let conn = &mut state.db.get_conn()?;
    let event = crate::db::event::event_info(conn, session.event_id)?;
    Ok(Json(event))
}

pub async fn event_note_edit(State(state): State<AppState>, session: EventSession, note: String) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::db::event::event_note_edit(conn, session.event_id, &note)?;
    Ok(())
}

pub async fn attendance_presence_pool(
    State(state): State<AppState>,
    session: EventSession,
    Query(query): Query<EventServiceRoleQuery>,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    let users =
        crate::db::event::attendance::event_attendance_presence_pool(conn, session.event_id, &query.role, true)?;
    Ok(Json(users))
}

pub async fn attendance_presence_list(
    State(state): State<AppState>,
    session: EventSession,
    Query(query): Query<EventServiceRoleQuery>,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    let users = crate::db::event::attendance::event_attendance_presence_list(conn, session.event_id, &query.role)?;
    Ok(Json(users))
}

pub async fn attendance_presence_add(
    State(state): State<AppState>,
    session: EventSession,
    Query(query): Query<EventServicePresenceQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    let pool = crate::db::event::attendance::event_attendance_presence_pool(conn, session.event_id, &query.role, true)?;

    if !pool.iter().any(|user| user.id == query.user_id) {
        return Err(Error::new(ErrorKind::Protected, "User is not in the presence pool"));
    }

    crate::db::event::attendance::event_attendance_presence_add(conn, session.event_id, query.user_id, &query.role)?;
    Ok(())
}

pub async fn attendance_presence_remove(
    State(state): State<AppState>,
    session: EventSession,
    Query(query): Query<EventServicePresenceQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::db::event::attendance::event_attendance_presence_remove(conn, session.event_id, query.user_id, &query.role)?;
    Ok(())
}
