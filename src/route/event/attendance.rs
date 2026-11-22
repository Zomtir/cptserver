use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::{Confirmation, User};
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;

#[derive(Deserialize)]
pub struct AttendanceRoleQuery {
    pub role: String,
}

#[derive(Deserialize)]
pub struct AttendanceQuery {
    pub user_id: u64,
    pub role: String,
    pub access: Option<bool>,
}

fn require_attendance_read(conn: &mut mysql::PooledConn, session: &UserSession, event_id: u64) -> Result<()> {
    crate::permission::require_right(session.right.right_event_read).or(crate::permission::require_event_owner(
        conn,
        event_id,
        session.user.id,
    ))
}

fn require_attendance_write(conn: &mut mysql::PooledConn, session: &UserSession, event_id: u64) -> Result<()> {
    crate::permission::require_right(session.right.right_event_write).or(crate::permission::require_event_owner(
        conn,
        event_id,
        session.user.id,
    ))
}

pub async fn registration_list(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<AttendanceRoleQuery>,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    require_attendance_read(conn, &session, event_id)?;

    let users = crate::db::event::attendance::event_attendance_registration_list(conn, event_id, query.role)?;
    Ok(Json(users))
}

pub async fn filter_list(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<AttendanceRoleQuery>,
) -> Result<Json<Vec<(User, bool)>>> {
    let conn = &mut state.db.get_conn()?;
    require_attendance_read(conn, &session, event_id)?;

    let filters = crate::db::event::attendance::event_attendance_filter_list(conn, event_id, query.role)?;
    Ok(Json(filters))
}

pub async fn filter_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<AttendanceQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    require_attendance_write(conn, &session, event_id)?;

    let access = query
        .access
        .ok_or_else(|| Error::new(ErrorKind::Missing, "Attendance filter access is missing"))?;
    crate::db::event::attendance::event_attendance_filter_edit(conn, event_id, query.user_id, query.role, access)?;
    Ok(())
}

pub async fn filter_remove(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<AttendanceQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    require_attendance_write(conn, &session, event_id)?;

    crate::db::event::attendance::event_attendance_filter_remove(conn, event_id, query.user_id, query.role)?;
    Ok(())
}

pub async fn presence_pool(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<AttendanceRoleQuery>,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    require_attendance_read(conn, &session, event_id)?;

    let users = crate::db::event::attendance::event_attendance_presence_pool(conn, event_id, &query.role, true)?;
    Ok(Json(users))
}

pub async fn presence_list(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<AttendanceRoleQuery>,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    require_attendance_read(conn, &session, event_id)?;

    let users = crate::db::event::attendance::event_attendance_presence_list(conn, event_id, &query.role)?;
    Ok(Json(users))
}

pub async fn presence_add(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<AttendanceQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    require_attendance_write(conn, &session, event_id).or(crate::permission::is_self(&session, Some(query.user_id)))?;

    let pool = crate::db::event::attendance::event_attendance_presence_pool(conn, event_id, &query.role, true)?;
    if !pool.iter().any(|user| user.id == query.user_id) {
        return Err(Error::new(ErrorKind::Protected, "User is not in the presence pool"));
    }

    crate::db::event::attendance::event_attendance_presence_add(conn, event_id, query.user_id, &query.role)?;
    Ok(())
}

pub async fn presence_remove(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<AttendanceQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    require_attendance_write(conn, &session, event_id).or(crate::permission::is_self(&session, Some(query.user_id)))?;

    crate::db::event::attendance::event_attendance_presence_remove(conn, event_id, query.user_id, &query.role)?;
    Ok(())
}

pub async fn presence_true(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<AttendanceQuery>,
) -> Result<Json<bool>> {
    let conn = &mut state.db.get_conn()?;
    require_attendance_read(conn, &session, event_id).or(crate::permission::is_self(&session, Some(query.user_id)))?;

    let condition =
        crate::db::event::attendance::event_attendance_presence_true(conn, event_id, query.user_id, &query.role)?;
    Ok(Json(condition))
}

/* WIP */

pub async fn registration_info(
    State(state): State<AppState>,
    session: UserSession,
    event_id: u64,
    role: String,
) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    // TODO check if you can register (requirement)

    let status =
        crate::db::event::attendance::event_attendance_registration_info(conn, event_id, session.user.id, role)?;
    Ok(status.to_string())
}

pub fn event_attendance_registration_edit(
    State(state): State<AppState>,
    session: UserSession,
    event_id: u64,
    role: String,
    status: Confirmation,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    // TODO check if you can register (requirement)

    match status {
        Confirmation::Null => {
            crate::db::event::attendance::event_attendance_registration_remove(conn, event_id, session.user.id, role)?
        }
        _ => crate::db::event::attendance::event_attendance_registration_edit(
            conn,
            event_id,
            session.user.id,
            role,
            status,
        )?,
    }
    Ok(())
}
