use rocket::serde::json::Json;

use crate::common::{Course, User};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/mod/course_responsibility?<active>&<public>")]
pub fn course_responsibility(session: UserSession, active: Option<bool>, public: Option<bool>) -> Result<Json<Vec<Course>>, Error> {
    match crate::db_course::course_list(Some(session.user.id), active, public)? {
        courses => Ok(Json(courses)),
    }
}

#[rocket::get("/mod/course_moderator_list?<course_id>")]
pub fn course_moderator_list(session: UserSession, course_id: i64) -> Result<Json<Vec<User>>, Error> {
    match crate::db_course::course_moderator_true(course_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    match crate::db_course::course_moderator_list(course_id)? {
        moderators => Ok(Json(moderators)),
    }
}

#[rocket::head("/mod/course_moderator_add?<course_id>&<user_id>")]
pub fn course_moderator_add(session: UserSession, course_id: i64, user_id: i64) -> Result<(), Error> {
    match crate::db_course::course_moderator_true(course_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    match crate::db_course::course_moderator_add(course_id, user_id) {
        None => Err(Error::DatabaseError),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/mod/course_moderator_remove?<course_id>&<user_id>")]
pub fn course_moderator_remove(session: UserSession, course_id: i64, user_id: i64) -> Result<(), Error> {
    match crate::db_course::course_moderator_true(course_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    match crate::db_course::course_moderator_remove(course_id, user_id) {
        None => Err(Error::DatabaseError),
        Some(..) => Ok(()),
    }
}
