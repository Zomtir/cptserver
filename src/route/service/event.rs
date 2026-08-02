use crate::common::{Event, User};
use crate::error::{Error, ErrorKind, Result};
use crate::session::EventSession;
use crate::AppState;
use axum::extract::State;
use axum::Json;

pub fn event_info(State(state): State<AppState>, session: EventSession) -> Result<Json<Event>> {
    let conn = &mut state.db.get_conn()?;
    Ok(Json(crate::db::event::event_info(conn, session.event_id)?))
}

pub fn event_note_edit(State(state): State<AppState>, session: EventSession, note: String) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::db::event::event_note_edit(conn, session.event_id, &note)?;
    Ok(())
}

pub fn event_attendance_presence_pool(
    State(state): State<AppState>,
    session: EventSession,
    role: String,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    let users = crate::db::event::attendance::event_attendance_presence_pool(conn, session.event_id, &role, true)?;
    Ok(Json(users))
}

pub fn event_attendance_presence_list(
    State(state): State<AppState>,
    session: EventSession,
    role: String,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    let users = crate::db::event::attendance::event_attendance_presence_list(conn, session.event_id, &role)?;
    Ok(Json(users))
}

pub fn event_attendance_presence_add(
    State(state): State<AppState>,
    session: EventSession,
    user_id: u64,
    role: String,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    let pool = crate::db::event::attendance::event_attendance_presence_pool(conn, session.event_id, &role, true)?;

    if !pool.iter().any(|user| user.id == user_id) {
        return Err(Error::new(ErrorKind::Protected, "User is not in the presence pool"));
    }
    crate::db::event::attendance::event_attendance_presence_add(conn, session.event_id, user_id, &role)
}

pub fn event_attendance_presence_remove(
    State(state): State<AppState>,
    session: EventSession,
    user_id: u64,
    role: String,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::db::event::attendance::event_attendance_presence_remove(conn, session.event_id, user_id, &role)
}
