use rocket::serde::json::Json;

use crate::common::Team;
use crate::error::{ErrorKind, Result};
use crate::session::UserSession;

#[rocket::get("/admin/course_attendance_sieve_list?<course_id>&<role>")]
pub fn sieve_list(session: UserSession, course_id: u32, role: String) -> Result<Json<Vec<(Team, bool)>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_read {
        return Err(ErrorKind::RightCourseMissing);
    };

    let teams = crate::db::course::attendance::sieve_list(conn, course_id, role)?;
    Ok(Json(teams))
}

#[rocket::head("/admin/course_attendance_sieve_edit?<course_id>&<team_id>&<role>&<access>")]
pub fn sieve_edit(session: UserSession, course_id: u32, team_id: u64, role: String, access: bool) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_write {
        return Err(ErrorKind::RightCourseMissing);
    };

    crate::db::course::attendance::sieve_edit(conn, course_id, team_id, role, access)?;
    Ok(())
}

#[rocket::head("/admin/course_attendance_sieve_remove?<course_id>&<team_id>&<role>")]
pub fn sieve_remove(session: UserSession, course_id: u32, team_id: u64, role: String) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_course_write {
        return Err(ErrorKind::RightCourseMissing);
    };

    crate::db::course::attendance::sieve_remove(conn, course_id, team_id, role)?;
    Ok(())
}
