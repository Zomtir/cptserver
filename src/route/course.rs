use axum::extract::{Path, State};
use axum::Json;

use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub async fn course_availability(
    State(state): State<AppState>,
    session: UserSession,
    Path(user_id): Path<u64>,
) -> Result<Json<Vec<crate::common::Course>>> {
    crate::permission::require_right(session.right.right_event_read)
        .or(crate::permission::is_self(&session, Some(user_id)))?;

    let conn = &mut state.db.get_conn()?;
    let courses = crate::db::course::course_available(conn, user_id)?;
    Ok(Json(courses))
}