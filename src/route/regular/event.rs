use crate::common::{Acceptance, Confirmation, Event, Occurrence, WebBool, WebDateTime};
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use axum::Json;

pub fn event_list(
    _session: UserSession,
    begin: Option<WebDateTime>,
    end: Option<WebDateTime>,
    location_id: Option<u64>,
    occurrence: Option<Occurrence>,
    acceptance: Option<Acceptance>,
    course_true: Option<WebBool>,
    course_id: Option<u32>,
) -> Result<Json<Vec<Event>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;

    let begin = begin.map(|dt| dt.to_naive());
    let end = end.map(|dt| dt.to_naive());
    crate::utils::event::verify_event_search_window(begin, end)?;

    let events = crate::db::event::event_list(
        conn,
        begin,
        end,
        location_id,
        occurrence,
        acceptance,
        course_true.map(|b| b.to_bool()),
        course_id,
        None,
    )?;
    Ok(Json(events))
}

pub fn event_create(session: UserSession, mut event: Json<Event>) -> Result<String> {
    crate::utils::event::validate_event_dates(&mut event)?;
    let conn = &mut crate::utils::db::get_db_conn()?;

    let event_id = crate::db::event::event_create(conn, &event, &Acceptance::Draft, None)?;
    crate::db::event::owner::event_owner_add(conn, event_id, session.user.id)?;
    Ok(event_id.to_string())
}

pub fn event_owner_true(session: UserSession, event_id: u64) -> Result<Json<bool>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let condition = crate::db::event::owner::event_owner_true(conn, event_id, session.user.id)?;
    Ok(Json(condition))
}

pub fn event_moderator_true(session: UserSession, event_id: u64) -> Result<Json<bool>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let condition = crate::db::event::moderator::event_moderator_true(conn, event_id, session.user.id)?;
    Ok(Json(condition))
}

pub fn event_attendance_presence_true(session: UserSession, event_id: u64, role: String) -> Result<Json<bool>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let condition =
        crate::db::event::attendance::event_attendance_presence_true(conn, event_id, session.user.id, &role)?;
    Ok(Json(condition))
}

pub fn event_attendance_presence_add(session: UserSession, event_id: u64, role: String) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let pool = crate::db::event::attendance::event_attendance_presence_pool(conn, event_id, &role, true)?;

    if !pool.iter().any(|user| user.id == session.user.id) {
        return Err(Error::new(ErrorKind::Protected, "User is not in the presence pool"));
    }

    crate::db::event::attendance::event_attendance_presence_add(conn, event_id, session.user.id, &role)?;
    Ok(())
}

pub fn event_attendance_presence_remove(session: UserSession, event_id: u64, role: String) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::db::event::attendance::event_attendance_presence_remove(conn, event_id, session.user.id, &role)?;
    Ok(())
}

pub fn event_bookmark_true(session: UserSession, event_id: u64) -> Result<Json<bool>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    // TODO check if you can participate

    let bookmark = crate::db::event::event_bookmark_true(conn, event_id, session.user.id)?;
    Ok(Json(bookmark))
}

pub fn event_bookmark_edit(session: UserSession, event_id: u64, bookmark: bool) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    // TODO check if you can participate

    match bookmark {
        true => crate::db::event::event_bookmark_add(conn, event_id, session.user.id)?,
        false => crate::db::event::event_bookmark_remove(conn, event_id, session.user.id)?,
    }
    Ok(())
}

pub fn event_attendance_registration_info(session: UserSession, event_id: u64, role: String) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    // TODO check if you can register (requirement)

    let status =
        crate::db::event::attendance::event_attendance_registration_info(conn, event_id, session.user.id, role)?;
    Ok(status.to_string())
}

pub fn event_attendance_registration_edit(
    session: UserSession,
    event_id: u64,
    role: String,
    status: Confirmation,
) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
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
