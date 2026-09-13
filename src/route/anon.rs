use crate::AppState;
use axum::extract::State;
use axum::Json;

use crate::common::{Club, Course, Skill};

use crate::error::{Error, ErrorKind, Result};

pub fn skill_list(State(state): State<AppState>) -> Result<Json<Vec<Skill>>> {
    let conn = &mut state.db.get_conn()?;
    let skills = crate::db::skill::skill_list(conn)?;
    Ok(Json(skills))
}

pub fn club_list(State(state): State<AppState>) -> Result<Json<Vec<Club>>> {
    let conn = &mut state.db.get_conn()?;
    let clubs = crate::db::club::club_list(conn)?;
    Ok(Json(clubs))
}

pub fn club_image(State(state): State<AppState>, club_id: u32) -> Result<Vec<u8>> {
    let conn = &mut state.db.get_conn()?;
    let club = crate::db::club::club_info(conn, club_id)?;

    let full_path = match club.image_url {
        None => crate::fs::get_resources_path().join("club_image_placeholder.png"),
        Some(url) => crate::fs::get_data_path().join(&format!("clubs/{}", url)),
    };

    std::fs::read(full_path).map_err(|_| Error::new(ErrorKind::Filesystem, "Failed to read club image"))
}

pub fn club_banner(State(state): State<AppState>, club_id: u32) -> Result<Vec<u8>> {
    let conn = &mut state.db.get_conn()?;
    let club = crate::db::club::club_info(conn, club_id)?;

    let full_path = match club.banner_url {
        None => crate::fs::get_resources_path().join("club_banner_placeholder.png"),
        Some(url) => crate::fs::get_data_path().join(&format!("clubs/{}", url)),
    };

    std::fs::read(full_path).map_err(|_| Error::new(ErrorKind::Filesystem, "Failed to read club banner"))
}

pub fn course_list(State(state): State<AppState>) -> Result<Json<Vec<Course>>> {
    let conn = &mut state.db.get_conn()?;
    let courses = crate::db::course::course_list(conn, None, Some(true), Some(true))?;
    Ok(Json(courses))
}
