use rocket::serde::json::Json;

use crate::common::Team;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/course_leader_sieve_list?<course_id>")]
pub fn sieve_list(session: UserSession, course_id: u32) -> Result<Json<Vec<(Team, bool)>>, Error> {
    if !session.right.right_course_read {
        return Err(Error::RightCourseMissing);
    };

    let teams = crate::db::course::leader::sieve_list(course_id)?;
    Ok(Json(teams))
}

#[rocket::head("/admin/course_leader_sieve_edit?<course_id>&<team_id>&<access>")]
pub fn sieve_edit(session: UserSession, course_id: u32, team_id: u64, access: bool) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db::course::leader::sieve_edit(course_id, team_id, access)?;
    Ok(())
}

#[rocket::head("/admin/course_leader_sieve_remove?<course_id>&<team_id>")]
pub fn sieve_remove(session: UserSession, course_id: u32, team_id: u64) -> Result<(), Error> {
    if !session.right.right_course_write {
        return Err(Error::RightCourseMissing);
    };

    crate::db::course::leader::sieve_remove(course_id, team_id)?;
    Ok(())
}
