use rocket::serde::json::Json;

use crate::common::Course;
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
