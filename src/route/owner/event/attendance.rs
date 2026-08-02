use crate::common::User;
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;

use axum::extract::State;
use axum::Json;

pub fn registration_list(
    State(state): State<AppState>,
    session: UserSession,
    event_id: u64,
    role: String,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    let users = crate::db::event::attendance::event_attendance_registration_list(conn, event_id, role)?;
    Ok(Json(users))
}

pub fn filter_list(
    State(state): State<AppState>,
    session: UserSession,
    event_id: u64,
    role: String,
) -> Result<Json<Vec<(User, bool)>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    let filters = crate::db::event::attendance::event_attendance_filter_list(conn, event_id, role)?;
    Ok(Json(filters))
}

pub fn filter_edit(
    State(state): State<AppState>,
    session: UserSession,
    event_id: u64,
    user_id: u64,
    role: String,
    access: bool,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    crate::db::event::attendance::event_attendance_filter_edit(conn, event_id, user_id, role, access)?;
    Ok(())
}

pub fn filter_remove(
    State(state): State<AppState>,
    session: UserSession,
    event_id: u64,
    user_id: u64,
    role: String,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    crate::db::event::attendance::event_attendance_filter_remove(conn, event_id, user_id, role)?;
    Ok(())
}

pub fn presence_pool(
    State(state): State<AppState>,
    session: UserSession,
    event_id: u64,
    role: String,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    let users = crate::db::event::attendance::event_attendance_presence_pool(conn, event_id, &role, true)?;
    Ok(Json(users))
}

pub fn presence_list(
    State(state): State<AppState>,
    session: UserSession,
    event_id: u64,
    role: String,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    let users = crate::db::event::attendance::event_attendance_presence_list(conn, event_id, &role)?;
    Ok(Json(users))
}

pub fn presence_add(
    State(state): State<AppState>,
    session: UserSession,
    event_id: u64,
    user_id: u64,
    role: String,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    let pool = crate::db::event::attendance::event_attendance_presence_pool(conn, event_id, &role, true)?;

    if !pool.iter().any(|user| user.id == user_id) {
        return Err(Error::new(ErrorKind::Protected, "User is not in the presence pool"));
    }

    crate::db::event::attendance::event_attendance_presence_add(conn, event_id, user_id, &role)?;
    Ok(())
}

pub fn presence_remove(
    State(state): State<AppState>,
    session: UserSession,
    event_id: u64,
    user_id: u64,
    role: String,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;

    crate::db::event::attendance::event_attendance_presence_remove(conn, event_id, user_id, &role)?;
    Ok(())
}
