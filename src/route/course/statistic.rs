use axum::extract::{Path, State};
use axum::Json;

use crate::common::{Event, User};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub async fn course_statistic_class(
    State(state): State<AppState>,
    session: UserSession,
    Path(course_id): Path<u32>,
) -> Result<Json<Vec<(Event, u64, u64, u64, u64)>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let stats = crate::db::course::course_statistic_class(conn, course_id)?;
    Ok(Json(stats))
}

pub async fn course_statistic_attendance(
    State(state): State<AppState>,
    session: UserSession,
    Path((course_id, role)): Path<(u32, String)>,
) -> Result<Json<Vec<(User, u64)>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let stats = crate::db::course::course_statistic_attendance(conn, course_id, role)?;
    Ok(Json(stats))
}

pub async fn course_statistic_attendance_user(
    State(state): State<AppState>,
    session: UserSession,
    Path((course_id, user_id, role)): Path<(u32, u64, String)>,
) -> Result<Json<Vec<Event>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let stats = crate::db::course::course_statistic_attendance_user(conn, course_id, user_id, role)?;
    Ok(Json(stats))
}
