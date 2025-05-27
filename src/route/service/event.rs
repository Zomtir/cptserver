use rocket::serde::json::Json;

use crate::common::{Event, User};
use crate::error::{ErrorKind, Result};
use crate::session::EventSession;

#[rocket::get("/service/event_info")]
pub fn event_info(session: EventSession) -> Result<Json<Event>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    Ok(Json(crate::db::event::event_info(conn, session.event_id)?))
}

#[rocket::post("/service/event_note_edit", format = "text/plain", data = "<note>")]
pub fn event_note_edit(session: EventSession, note: String) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::db::event::event_note_edit(conn, session.event_id, &note)?;
    Ok(())
}

#[rocket::get("/service/event_attendance_presence_pool?<role>")]
pub fn event_attendance_presence_pool(session: EventSession, role: String) -> Result<Json<Vec<User>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let users = crate::db::event::attendance::event_attendance_presence_pool(conn, session.event_id, &role, true)?;
    Ok(Json(users))
}

#[rocket::get("/service/event_attendance_presence_list?<role>")]
pub fn event_attendance_presence_list(session: EventSession, role: String) -> Result<Json<Vec<User>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let users = crate::db::event::attendance::event_attendance_presence_list(conn, session.event_id, &role)?;
    Ok(Json(users))
}

#[rocket::head("/service/event_attendance_presence_add?<user_id>&<role>")]
pub fn event_attendance_presence_add(session: EventSession, user_id: u64, role: String) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let pool = crate::db::event::attendance::event_attendance_presence_pool(conn, session.event_id, &role, true)?;

    if !pool.iter().any(|user| user.id == user_id) {
        return Err(ErrorKind::EventPresenceForbidden);
    }
    crate::db::event::attendance::event_attendance_presence_add(conn, session.event_id, user_id, &role)
}

#[rocket::head("/service/event_attendance_presence_remove?<user_id>&<role>")]
pub fn event_attendance_presence_remove(session: EventSession, user_id: u64, role: String) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::db::event::attendance::event_attendance_presence_remove(conn, session.event_id, user_id, &role)
}
