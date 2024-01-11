use rocket::serde::json::Json;

use crate::common::Slot;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/regular/class_list?<course_id>")]
pub fn class_list(session: UserSession, course_id: i64) -> Result<Json<Vec<Slot>>, Error> {
    // TODO check if course is public
    // TODO check if member is part of course

    match crate::db_slot::list_slots(None, None, None, None, Some(course_id), None)? {
        slots => Ok(Json(slots)),
    }
}
