use axum::Json;

use crate::common::Team;
use crate::error::Result;
use crate::session::UserSession;

pub fn sieve_list(session: UserSession, course_id: u32, role: String) -> Result<Json<Vec<(Team, bool)>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let teams = crate::db::course::attendance::sieve_list(conn, course_id, role)?;
    Ok(Json(teams))
}

pub fn sieve_edit(session: UserSession, course_id: u32, team_id: u64, role: String, access: bool) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::attendance::sieve_edit(conn, course_id, team_id, role, access)?;
    Ok(())
}

pub fn sieve_remove(session: UserSession, course_id: u32, team_id: u64, role: String) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::attendance::sieve_remove(conn, course_id, team_id, role)?;
    Ok(())
}
