use axum::Json;

use crate::common::User;
use crate::error::Result;
use crate::session::UserSession;

pub fn course_moderator_list(session: UserSession, course_id: u32) -> Result<Json<Vec<User>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let moderators = crate::db::course::moderator::course_moderator_list(conn, course_id)?;
    Ok(Json(moderators))
}

pub fn course_moderator_add(session: UserSession, course_id: u32, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::moderator::course_moderator_add(conn, course_id, user_id)?;
    Ok(())
}

pub fn course_moderator_remove(session: UserSession, course_id: u32, user_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::moderator::course_moderator_remove(conn, course_id, user_id)?;
    Ok(())
}
