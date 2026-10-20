use crate::AppState;
use axum::extract::State;
use axum::Json;

use crate::common::Course;

use crate::error::Result;

pub fn course_list(State(state): State<AppState>) -> Result<Json<Vec<Course>>> {
    let conn = &mut state.db.get_conn()?;
    let courses = crate::db::course::course_list(conn, None, Some(true), Some(true))?;
    Ok(Json(courses))
}
