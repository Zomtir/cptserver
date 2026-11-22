use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::User;
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;

pub async fn course_info(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
) -> Result<Json<Option<crate::common::Course>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_read).or(crate::permission::require_event_owner(
        conn,
        event_id,
        session.user.id,
    ))?;

    let course = crate::db::event::event_course_info(conn, event_id)?;
    Ok(Json(course))
}

#[derive(Deserialize)]
pub struct EventCourseEditQuery {
    pub course_id: Option<u32>,
}

pub async fn course_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<EventCourseEditQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_write).or(crate::permission::require_event_owner(
        conn,
        event_id,
        session.user.id,
    ))?;

    crate::permission::require_right(session.right.right_course_write).or_else(|_| -> Result<()> {
        if let Some(old_course) = crate::db::event::event_course_info(conn, event_id)? {
            crate::permission::require_course_moderator(conn, Some(old_course.id), Some(session.user.id))?;
        }

        if let Some(new_course) = query.course_id {
            crate::permission::require_course_moderator(conn, Some(new_course), Some(session.user.id))?;
        }

        Ok(())
    })?;

    crate::db::event::event_course_edit(conn, event_id, query.course_id)?;
    Ok(())
}

pub async fn moderator_true(
    State(state): State<AppState>,
    session: UserSession,
    Path((event_id, user_id)): Path<(u64, u64)>,
) -> Result<Json<bool>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_read)
        .or(crate::permission::require_event_owner(conn, event_id, user_id))
        .or(crate::permission::is_self(&session, Some(user_id)))?;

    let condition = crate::db::event::moderator::event_moderator_true(conn, event_id, user_id)?;
    Ok(Json(condition))
}
