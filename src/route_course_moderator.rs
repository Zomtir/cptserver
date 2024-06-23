use rocket::serde::json::Json;

use crate::common::{Course, User, WebBool};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/mod/course_responsibility?<active>&<public>")]
pub fn course_responsibility(
    session: UserSession,
    active: Option<WebBool>,
    public: Option<WebBool>,
) -> Result<Json<Vec<Course>>, Error> {
    let courses = crate::db::course::course_list(
        Some(session.user.id),
        active.map(|b| b.to_bool()),
        public.map(|b| b.to_bool()),
    )?;
    Ok(Json(courses))
}

#[rocket::get("/mod/course_moderator_list?<course_id>")]
pub fn course_moderator_list(session: UserSession, course_id: u64) -> Result<Json<Vec<User>>, Error> {
    match crate::db::course::moderator::course_moderator_true(course_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    let moderators = crate::db::course::moderator::course_moderator_list(course_id)?;
    Ok(Json(moderators))
}

#[rocket::head("/mod/course_moderator_add?<course_id>&<user_id>")]
pub fn course_moderator_add(session: UserSession, course_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db::course::moderator::course_moderator_true(course_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::db::course::moderator::course_moderator_add(course_id, user_id)?;
    Ok(())
}

#[rocket::head("/mod/course_moderator_remove?<course_id>&<user_id>")]
pub fn course_moderator_remove(session: UserSession, course_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db::course::moderator::course_moderator_true(course_id, session.user.id)? {
        false => return Err(Error::CourseModeratorPermission),
        true => (),
    };

    crate::db::course::moderator::course_moderator_remove(course_id, user_id)?;
    Ok(())
}
