use axum::Json;

use crate::common::{Club, Course, Location, Organisation, Skill};

use crate::error::{Error, ErrorKind, Result};

pub fn location_list() -> Result<Json<Vec<Location>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let locations = crate::db::location::location_list(conn)?;
    Ok(Json(locations))
}

pub fn organisation_list() -> Result<Json<Vec<Organisation>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let organisations = crate::db::organisation::organisation_list(conn)?;
    Ok(Json(organisations))
}

pub fn skill_list() -> Result<Json<Vec<Skill>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let skills = crate::db::skill::skill_list(conn)?;
    Ok(Json(skills))
}

pub fn club_list() -> Result<Json<Vec<Club>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let clubs = crate::db::club::club_list(conn)?;
    Ok(Json(clubs))
}

pub fn club_image(club_id: u32) -> Result<Vec<u8>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let club = crate::db::club::club_info(conn, club_id)?;

    let full_path = match club.image_url {
        None => crate::fs::get_resources_path().join("club_image_placeholder.png"),
        Some(url) => crate::fs::get_data_path().join(&format!("clubs/{}", url)),
    };

    std::fs::read(full_path).map_err(|_| Error::new(ErrorKind::Filesystem, "Failed to read club image"))
}

pub fn club_banner(club_id: u32) -> Result<Vec<u8>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let club = crate::db::club::club_info(conn, club_id)?;

    let full_path = match club.banner_url {
        None => crate::fs::get_resources_path().join("club_banner_placeholder.png"),
        Some(url) => crate::fs::get_data_path().join(&format!("clubs/{}", url)),
    };

    std::fs::read(full_path).map_err(|_| Error::new(ErrorKind::Filesystem, "Failed to read club banner"))
}

pub fn course_list() -> Result<Json<Vec<Course>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let courses = crate::db::course::course_list(conn, None, Some(true), Some(true))?;
    Ok(Json(courses))
}

pub fn user_salt(user_key: String) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let salt = crate::db::user::user_key_salt_value(conn, &user_key);

    // If the user does not exist, just return a "random" salt to prevent data scraping
    match salt {
        Err(_) => Ok(hex::encode(crate::common::hash128_string(&user_key))),
        Ok(salt) => Ok(hex::encode(salt)),
    }
}
