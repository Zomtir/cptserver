use rocket::serde::json::Json;

use crate::common::User;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/course_moderator_list?<course_id>")]
pub fn course_moderator_list(session: UserSession, course_id: u64) -> Result<Json<Vec<User>>, Error> {
    if !session.right.right_course_read {
        return Err(Error::RightCourseMissing);
    };

    let moderators = crate::db::course::moderator::course_moderator_list(course_id)?;
    Ok(Json(moderators))
}

#[rocket::head("/admin/course_moderator_add?<course_id>&<user_id>")]
pub fn course_moderator_add(session: UserSession, course_id: u64, user_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db::course::moderator::course_moderator_add(course_id, user_id)?;
    Ok(())
}

#[rocket::head("/admin/course_moderator_remove?<course_id>&<user_id>")]
pub fn course_moderator_remove(session: UserSession, course_id: u64, user_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db::course::moderator::course_moderator_remove(course_id, user_id)?;
    Ok(())
}