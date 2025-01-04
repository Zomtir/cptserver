use rocket::serde::json::Json;

use crate::common::{Acceptance, Event};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/mod/event_list?<course_id>")]
pub fn event_list(session: UserSession, course_id: u32) -> Result<Json<Vec<Event>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !crate::db::course::moderator::course_moderator_true(conn, course_id, session.user.id)? {
        return Err(Error::CourseModeratorPermission);
    };

    let events = crate::db::event::event_list(conn, None, None, None, None, None, Some(true), Some(course_id), None)?;
    Ok(Json(events))
}

#[rocket::post("/mod/event_create?<course_id>", format = "application/json", data = "<event>")]
pub fn event_create(session: UserSession, course_id: u32, mut event: Json<Event>) -> Result<String, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !crate::db::course::moderator::course_moderator_true(conn, course_id, session.user.id)? {
        return Err(Error::CourseModeratorPermission);
    };

    crate::utils::event::validate_event_dates(&mut event)?;

    let id = crate::db::event::event_create(conn, &event, &Acceptance::Accepted, Some(course_id))?;
    Ok(id.to_string())
}

#[rocket::post("/mod/event_edit?<event_id>", format = "application/json", data = "<event>")]
pub fn event_edit(session: UserSession, event_id: u64, mut event: Json<Event>) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !crate::db::event::event_moderator_true(conn, event_id, session.user.id)?{
        return Err(Error::CourseModeratorPermission);
    };

    crate::utils::event::validate_event_dates(&mut event)?;

    crate::db::event::event_edit(conn, event_id, &event)?;
    Ok(())
}

#[rocket::post("/mod/event_edit_password?<event_id>", format = "text/plain", data = "<password>")]
pub fn event_edit_password(session: UserSession, event_id: u64, password: String) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !crate::db::event::event_moderator_true(conn, event_id, session.user.id)?{
        return Err(Error::CourseModeratorPermission);
    };

    let password = crate::utils::event::validate_clear_password(password)?;
    crate::db::event::event_password_edit(conn, event_id, password)?;
    Ok(())
}

#[rocket::head("/mod/event_delete?<event_id>")]
pub fn event_delete(session: UserSession, event_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !crate::db::event::event_moderator_true(conn, event_id, session.user.id)?{
        return Err(Error::CourseModeratorPermission);
    };

    crate::db::event::event_delete(conn, event_id)?;
    Ok(())
}
