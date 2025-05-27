use rocket::serde::json::Json;

use crate::common::Course;
use crate::error::Result;
use crate::session::UserSession;

/*
 * ROUTES
 */

#[rocket::get("/regular/course_availability")]
pub fn course_availability(session: UserSession) -> Result<Json<Vec<Course>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let courses = crate::db::course::course_available(conn, session.user.id)?;
    Ok(Json(courses))
}
