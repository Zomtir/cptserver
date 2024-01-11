use rocket::serde::json::Json;

use crate::common::Course;
use crate::error::Error;
use crate::session::UserSession;

/*
 * ROUTES
 */

#[rocket::get("/regular/course_availability")]
pub fn course_availability(session: UserSession) -> Result<Json<Vec<Course>>, Error> {
    match crate::db_course::available_courses(session.user.id)? {
        courses => Ok(Json(courses)),
    }
}
