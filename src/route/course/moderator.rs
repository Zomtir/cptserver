use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::User;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

use crate::common::{Course, WebBool};

pub async fn course_moderator_list(
    State(state): State<AppState>,
    session: UserSession,
    Path(course_id): Path<u32>,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)
        .or(crate::permission::require_course_moderator(conn, course_id, session.user.id))?;

    let moderators = crate::db::course::moderator::course_moderator_list(conn, course_id)?;
    Ok(Json(moderators))
}

pub async fn course_moderator_add(
    State(state): State<AppState>,
    session: UserSession,
    Path((course_id, user_id)): Path<(u32, u64)>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)
        .or(crate::permission::require_course_moderator(conn, course_id, session.user.id))?;

    crate::db::course::moderator::course_moderator_add(conn, course_id, user_id)?;
    Ok(())
}

pub async fn course_moderator_remove(
    State(state): State<AppState>,
    session: UserSession,
    Path((course_id, user_id)): Path<(u32, u64)>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)
        .or(crate::permission::require_course_moderator(conn, course_id, session.user.id))?;

    crate::db::course::moderator::course_moderator_remove(conn, course_id, user_id)?;
    Ok(())
}

#[derive(Deserialize)]
pub struct CourseResponsibilityQuery {
    pub active: Option<WebBool>,
    pub public: Option<WebBool>,
}

pub async fn course_responsibility(
    State(state): State<AppState>,
    session: UserSession,
    Path(user_id): Path<u64>,
    Query(query): Query<CourseResponsibilityQuery>,
) -> Result<Json<Vec<Course>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)
        .or(crate::permission::is_self(&session, Some(user_id)))?;

    let courses = crate::db::course::course_list(
        conn,
        Some(user_id),
        query.active.map(|b| b.to_bool()),
        query.public.map(|b| b.to_bool()),
    )?;
    Ok(Json(courses))
}