use crate::common::Course;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;
use axum::extract::State;
use axum::Json;

pub fn course_availability(State(state): State<AppState>, session: UserSession) -> Result<Json<Vec<Course>>> {
    let conn = &mut state.db.get_conn()?;
    let courses = crate::db::course::course_available(conn, session.user.id)?;
    Ok(Json(courses))
}
