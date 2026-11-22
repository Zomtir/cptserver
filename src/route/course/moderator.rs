use axum::extract::{Path, State};
use axum::Json;

use crate::common::User;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub async fn course_moderator_list(
    State(state): State<AppState>,
    session: UserSession,
    Path(course_id): Path<u32>,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read).or(
        crate::permission::require_course_moderator(conn, Some(course_id), Some(session.user.id)),
    )?;

    let moderators = crate::db::course::moderator::course_moderator_list(conn, course_id)?;
    Ok(Json(moderators))
}

pub async fn course_moderator_true(
    State(state): State<AppState>,
    session: UserSession,
    Path((course_id, user_id)): Path<(u32, u64)>,
) -> Result<Json<bool>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)
        .or(crate::permission::require_course_moderator(
            conn,
            Some(course_id),
            Some(session.user.id),
        ))
        .or(crate::permission::is_self(&session, Some(user_id)))?;

    let condition = crate::db::course::moderator::course_moderator_true(conn, course_id, user_id)?;
    Ok(Json(condition))
}

pub async fn course_moderator_add(
    State(state): State<AppState>,
    session: UserSession,
    Path((course_id, user_id)): Path<(u32, u64)>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write).or(
        crate::permission::require_course_moderator(conn, Some(course_id), Some(session.user.id)),
    )?;

    crate::db::course::moderator::course_moderator_add(conn, course_id, user_id)?;
    Ok(())
}

pub async fn course_moderator_remove(
    State(state): State<AppState>,
    session: UserSession,
    Path((course_id, user_id)): Path<(u32, u64)>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write).or(
        crate::permission::require_course_moderator(conn, Some(course_id), Some(session.user.id)),
    )?;

    crate::db::course::moderator::course_moderator_remove(conn, course_id, user_id)?;
    Ok(())
}
