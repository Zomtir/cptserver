use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::common::{User, Course};
use crate::session::UserSession;

#[rocket::get("/mod/course_responsiblity")]
pub fn course_responsiblity(session: UserSession) -> Result<Json<Vec<Course>>, ApiError> {
    match crate::db_course::responsible_courses(session.user.id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(courses) => Ok(Json(courses)),
    }
}

#[rocket::get("/mod/course_moderator_list?<course_id>")]
pub fn course_moderator_list(
    session: UserSession,
    course_id: u32,
) -> Result<Json<Vec<User>>, ApiError> {
    match crate::db_course::is_course_moderator(&course_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::COURSE_NO_MODERATOR),
        Some(true) => (),
    };

    match crate::db_course::list_course_moderators(&course_id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(moderators) => Ok(Json(moderators)),
    }
}

#[rocket::head("/mod/course_moderator_add?<course_id>&<user_id>")]
pub fn course_moderator_add(
    session: UserSession,
    course_id: u32,
    user_id: u32,
) -> Result<(), ApiError> {
    match crate::db_course::is_course_moderator(&course_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::COURSE_NO_MODERATOR),
        Some(true) => (),
    };

    match crate::db_course::add_course_moderator(course_id, user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/mod/course_moderator_remove?<course_id>&<user_id>")]
pub fn course_moderator_remove(
    session: UserSession,
    course_id: u32,
    user_id: u32,
) -> Result<(), ApiError> {
    match crate::db_course::is_course_moderator(&course_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::COURSE_NO_MODERATOR),
        Some(true) => (),
    };

    match crate::db_course::remove_course_moderator(course_id, user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}
