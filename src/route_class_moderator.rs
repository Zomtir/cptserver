use rocket::serde::json::Json;

use crate::common::Slot;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/mod/class_list?<course_id>")]
pub fn class_list(session: UserSession, course_id: i64) -> Result<Json<Vec<Slot>>, Error> {
    match crate::db_course::is_course_moderator(course_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    match crate::db_slot::list_slots(None, None, None, None, Some(course_id), None)? {
        slots => Ok(Json(slots)),
    }
}

#[rocket::post("/mod/class_create?<course_id>", format = "application/json", data = "<slot>")]
pub fn class_create(session: UserSession, course_id: i64, mut slot: Json<Slot>) -> Result<String, Error> {
    match crate::db_course::is_course_moderator(course_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::common::validate_slot_dates(&mut slot)?;

    match crate::db_slot::slot_create(&slot, "OCCURRING", Some(course_id))? {
        id => Ok(id.to_string()),
    }
}

#[rocket::post("/mod/class_edit?<slot_id>", format = "application/json", data = "<slot>")]
pub fn class_edit(session: UserSession, slot_id: i64, mut slot: Json<Slot>) -> Result<(), Error> {
    match crate::db_slot::is_slot_moderator(slot_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::common::validate_slot_dates(&mut slot)?;

    crate::db_slot::edit_slot(slot_id, &slot)?;
    Ok(())
}

#[rocket::post("/mod/class_edit_password?<slot_id>", format = "text/plain", data = "<password>")]
pub fn class_edit_password(session: UserSession, slot_id: i64, password: String) -> Result<(), Error> {
    match crate::db_slot::is_slot_moderator(slot_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::db_slot::edit_slot_password(slot_id, password)?;
    Ok(())
}

#[rocket::head("/mod/class_delete?<slot_id>")]
pub fn class_delete(session: UserSession, slot_id: i64) -> Result<(), Error> {
    match crate::db_slot::is_slot_moderator(slot_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::db_slot::slot_delete(slot_id)?;
    Ok(())
}
