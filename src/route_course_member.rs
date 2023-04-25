use rocket::serde::json::Json;

use crate::common::{Course, Slot};
use crate::error::Error;
use crate::session::UserSession;

/*
 * ROUTES
 */

#[rocket::get("/member/course_availiblity")]
pub fn course_availiblity(session: UserSession) -> Result<Json<Vec<Course>>, Error> {
    match crate::db_course::available_courses(session.user.id) {
        None => Err(Error::DatabaseError),
        Some(courses) => Ok(Json(courses)),
    }
}

#[rocket::get("/member/course_class_list?<course_id>")]
pub fn course_class_list(session: UserSession, course_id: i64) -> Result<Json<Vec<Slot>>, Error> {
    // TODO check if course is public
    // TODO check if member is part of course

    match crate::db_slot::list_slots(None, None, None, Some(course_id), None)? {
        slots => Ok(Json(slots)),
    }
}
