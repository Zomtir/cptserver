use rocket::serde::json::Json;

use crate::common::{Acceptance, Event};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/mod/event_list?<course_id>")]
pub fn event_list(session: UserSession, course_id: u32) -> Result<Json<Vec<Event>>, Error> {
    match crate::db::course::moderator::course_moderator_true(course_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    let events = crate::db::event::event_list(None, None, None, None, None, Some(true), Some(course_id), None)?;
    Ok(Json(events))
}

#[rocket::post("/mod/event_create?<course_id>", format = "application/json", data = "<event>")]
pub fn event_create(session: UserSession, course_id: u32, mut event: Json<Event>) -> Result<String, Error> {
    match crate::db::course::moderator::course_moderator_true(course_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::common::validate_event_dates(&mut event)?;

    let id = crate::db::event::event_create(&event, &Acceptance::Accepted, Some(course_id))?;
    Ok(id.to_string())
}

#[rocket::post("/mod/event_edit?<event_id>", format = "application/json", data = "<event>")]
pub fn event_edit(session: UserSession, event_id: u64, mut event: Json<Event>) -> Result<(), Error> {
    match crate::db::event::event_moderator_true(event_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::common::validate_event_dates(&mut event)?;

    crate::db::event::event_edit(event_id, &event)?;
    Ok(())
}

#[rocket::post("/mod/event_edit_password?<event_id>", format = "text/plain", data = "<password>")]
pub fn event_edit_password(session: UserSession, event_id: u64, password: String) -> Result<(), Error> {
    match crate::db::event::event_moderator_true(event_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::db::event::event_password_edit(event_id, password)?;
    Ok(())
}

#[rocket::head("/mod/event_delete?<event_id>")]
pub fn event_delete(session: UserSession, event_id: u64) -> Result<(), Error> {
    match crate::db::event::event_moderator_true(event_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::db::event::event_delete(event_id)?;
    Ok(())
}
