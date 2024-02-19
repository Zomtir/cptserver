use rocket::serde::json::Json;

use crate::common::Slot;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/regular/class_list?<course_id>")]
pub fn class_list(session: UserSession, course_id: i64) -> Result<Json<Vec<Slot>>, Error> {
    // TODO check if course is public
    // TODO check if member is part of course

    match crate::db_slot::list_slots(None, None, None, None, Some(true), Some(course_id), None)? {
        slots => Ok(Json(slots)),
    }
}

#[rocket::head("/regular/class_participant_add?<slot_id>")]
pub fn class_participant_add(session: UserSession, slot_id: i64) -> Result<(), Error> {
    // TODO check if course is public
    // TODO check if member is part of course

    crate::db_slot::slot_participant_add(slot_id, session.user.id)?;
    Ok(())
}

#[rocket::head("/regular/class_participant_remove?<slot_id>")]
pub fn class_participant_remove(session: UserSession, slot_id: i64) -> Result<(), Error> {
    // TODO check if course is public
    // TODO check if member is part of course

    crate::db_slot::slot_participant_remove(slot_id, session.user.id)?;
    Ok(())
}
