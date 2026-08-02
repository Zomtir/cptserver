use axum::extract::State;
use axum::Json;

use crate::common::User;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub fn course_moderator_list(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let moderators = crate::db::course::moderator::course_moderator_list(conn, course_id)?;
    Ok(Json(moderators))
}

pub fn course_moderator_add(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
    user_id: u64,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::moderator::course_moderator_add(conn, course_id, user_id)?;
    Ok(())
}

pub fn course_moderator_remove(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
    user_id: u64,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::moderator::course_moderator_remove(conn, course_id, user_id)?;
    Ok(())
}
