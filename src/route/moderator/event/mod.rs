use rocket::serde::json::Json;

use crate::common::{Acceptance, Event};
use crate::error::Result;
use crate::session::UserSession;

#[rocket::get("/mod/event_list?<course_id>")]
pub fn event_list(session: UserSession, course_id: u32) -> Result<Json<Vec<Event>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_course_moderator(conn, course_id, session.user.id)?;

    let events = crate::db::event::event_list(conn, None, None, None, None, None, Some(true), Some(course_id), None)?;
    Ok(Json(events))
}

#[rocket::post("/mod/event_create?<course_id>", format = "application/json", data = "<event>")]
pub fn event_create(session: UserSession, course_id: u32, mut event: Json<Event>) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_course_moderator(conn, course_id, session.user.id)?;

    crate::utils::event::validate_event_dates(&mut event)?;

    let id = crate::db::event::event_create(conn, &event, &Acceptance::Accepted, Some(course_id))?;
    Ok(id.to_string())
}

#[rocket::post("/mod/event_edit?<event_id>", format = "application/json", data = "<event>")]
pub fn event_edit(session: UserSession, event_id: u64, mut event: Json<Event>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_event_moderator(conn, event_id, session.user.id)?;

    crate::utils::event::validate_event_dates(&mut event)?;

    crate::db::event::event_edit(conn, event_id, &event)?;
    Ok(())
}

#[rocket::post("/mod/event_edit_password?<event_id>", format = "text/plain", data = "<password>")]
pub fn event_edit_password(session: UserSession, event_id: u64, password: String) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_event_moderator(conn, event_id, session.user.id)?;

    let password = crate::utils::event::validate_clear_password(password)?;
    crate::db::event::event_password_edit(conn, event_id, password)?;
    Ok(())
}

#[rocket::head("/mod/event_delete?<event_id>")]
pub fn event_delete(session: UserSession, event_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_event_moderator(conn, event_id, session.user.id)?;

    crate::db::event::event_delete(conn, event_id)?;
    Ok(())
}
