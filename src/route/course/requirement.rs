use axum::extract::{Path, State};
use axum::Json;

use crate::common::Requirement;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub async fn course_requirement_list(
    State(state): State<AppState>,
    session: UserSession,
    Path(course_id): Path<u32>,
) -> Result<Json<Vec<Requirement>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let reqs = crate::db::course::course_requirement_list(conn, course_id)?;
    Ok(Json(reqs))
}

pub async fn course_requirement_add(
    State(state): State<AppState>,
    session: UserSession,
    Path(course_id): Path<u32>,
    Json(requirement): Json<Requirement>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::course_requirement_add(conn, course_id, &requirement)?;
    Ok(())
}

pub async fn course_requirement_remove(
    State(state): State<AppState>,
    session: UserSession,
    Path(requirement_id): Path<u64>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::course_requirement_remove(conn, requirement_id)?;
    Ok(())
}
