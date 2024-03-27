use rocket::serde::json::Json;

use crate::common::Slot;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/mod/event_list?<course_id>")]
pub fn event_list(session: UserSession, course_id: u64) -> Result<Json<Vec<Slot>>, Error> {
    match crate::db_course::course_moderator_true(course_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    let slots = crate::db_slot::list_slots(None, None, None, None, Some(true), Some(course_id), None)?;
    Ok(Json(slots))
}

#[rocket::post("/mod/event_create?<course_id>", format = "application/json", data = "<slot>")]
pub fn event_create(session: UserSession, course_id: u64, mut slot: Json<Slot>) -> Result<String, Error> {
    match crate::db_course::course_moderator_true(course_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::common::validate_slot_dates(&mut slot)?;

    let id = crate::db_slot::slot_create(&slot, "OCCURRING", Some(course_id))?;
    Ok(id.to_string())
}

#[rocket::post("/mod/event_edit?<slot_id>", format = "application/json", data = "<slot>")]
pub fn event_edit(session: UserSession, slot_id: u64, mut slot: Json<Slot>) -> Result<(), Error> {
    match crate::db_slot::slot_moderator_true(slot_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::common::validate_slot_dates(&mut slot)?;

    crate::db_slot::edit_slot(slot_id, &slot)?;
    Ok(())
}

#[rocket::post("/mod/event_edit_password?<slot_id>", format = "text/plain", data = "<password>")]
pub fn event_edit_password(session: UserSession, slot_id: u64, password: String) -> Result<(), Error> {
    match crate::db_slot::slot_moderator_true(slot_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::db_slot::edit_slot_password(slot_id, password)?;
    Ok(())
}

#[rocket::head("/mod/event_delete?<slot_id>")]
pub fn event_delete(session: UserSession, slot_id: u64) -> Result<(), Error> {
    match crate::db_slot::slot_moderator_true(slot_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::db_slot::slot_delete(slot_id)?;
    Ok(())
}
