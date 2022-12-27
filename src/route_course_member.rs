use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::session::UserSession;
use crate::common::{Course, Slot};

/*
 * ROUTES
 */

 #[rocket::get("/member/course_availiblity")]
pub fn course_availiblity(session: UserSession) -> Result<Json<Vec<Course>>, ApiError> {
    match crate::db_course::get_course_availiblity(session.user.id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(courses) => Ok(Json(courses)),
    }
}

#[rocket::get("/member/course_class_list?<course_id>")]
pub fn course_class_list(session: UserSession, course_id: u32) -> Result<Json<Vec<Slot>>, ApiError> {
    // TODO check if course is public
    // TODO check if part of course

    match crate::db_course::get_course_class_list(course_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(slots) => Ok(Json(slots)),
    }
}
