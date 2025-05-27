extern crate lazy_static;

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::common::{Club, Course, Location, Organisation, Skill};

use crate::error::{ErrorKind, Result};

#[rocket::head("/status")]
pub fn status() -> Status {
    Status::Ok
}

#[rocket::get("/anon/location_list")]
pub fn location_list() -> Result<Json<Vec<Location>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let locations = crate::db::location::location_list(conn)?;
    Ok(Json(locations))
}

#[rocket::get("/anon/organisation_list")]
pub fn organisation_list() -> Result<Json<Vec<Organisation>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let organisations = crate::db::organisation::organisation_list(conn)?;
    Ok(Json(organisations))
}

#[rocket::get("/anon/skill_list")]
pub fn skill_list() -> Result<Json<Vec<Skill>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let skills = crate::db::skill::skill_list(conn)?;
    Ok(Json(skills))
}

#[rocket::get("/anon/club_list")]
pub fn club_list() -> Result<Json<Vec<Club>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let clubs = crate::db::club::club_list(conn)?;
    Ok(Json(clubs))
}

#[rocket::get("/anon/club_image?<club_id>")]
pub fn club_image(club_id: u32) -> Result<Vec<u8>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let club = crate::db::club::club_info(conn, club_id)?;

    let image_url = match club.image_url {
        None => "resources/club_image_placeholder.png".to_string(),
        Some(url) => format!("data/clubs/{}", url),
    };

    let local_path = crate::common::fs::local_path(&image_url)?;
    std::fs::read(local_path).map_err(|_| ErrorKind::Default)
}

#[rocket::get("/anon/club_banner?<club_id>")]
pub fn club_banner(club_id: u32) -> Result<Vec<u8>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let club = crate::db::club::club_info(conn, club_id)?;

    let banner_url = match club.banner_url {
        None => "resources/club_banner_placeholder.png".to_string(),
        Some(url) => format!("data/clubs/{}", url),
    };

    let local_path = crate::common::fs::local_path(&banner_url)?;
    std::fs::read(local_path).map_err(|_| ErrorKind::Default)
}

#[rocket::get("/anon/course_list")]
pub fn course_list() -> Result<Json<Vec<Course>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let courses = crate::db::course::course_list(conn, None, Some(true), Some(true))?;
    Ok(Json(courses))
}

#[rocket::get("/anon/user_salt?<user_key>")]
pub fn user_salt(user_key: String) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let salt = crate::db::user::user_key_salt_value(conn, &user_key);

    // If the user does not exist, just return a "random" salt to prevent data scraping
    match salt {
        Err(_) => Ok(hex::encode(crate::common::hash128_string(&user_key))),
        Ok(salt) => Ok(hex::encode(salt)),
    }
}
