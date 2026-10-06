use crate::AppState;
use axum::extract::State;
use axum::Json;

use crate::common::{Course, Skill};

use crate::error::{Error, ErrorKind, Result};

pub fn skill_list(State(state): State<AppState>) -> Result<Json<Vec<Skill>>> {
    let conn = &mut state.db.get_conn()?;
    let skills = crate::db::skill::skill_list(conn)?;
    Ok(Json(skills))
}

pub fn course_list(State(state): State<AppState>) -> Result<Json<Vec<Course>>> {
    let conn = &mut state.db.get_conn()?;
    let courses = crate::db::course::course_list(conn, None, Some(true), Some(true))?;
    Ok(Json(courses))
}
