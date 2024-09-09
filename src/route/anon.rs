extern crate lazy_static;

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::common::{Club, Course, Location, Organisation, Skill};

use crate::error::Error;

#[rocket::head("/status")]
pub fn status() -> Status {
    Status::Ok
}

#[rocket::get("/anon/location_list")]
pub fn location_list() -> Result<Json<Vec<Location>>, Error> {
    let locations = crate::db::location::location_list()?;
    Ok(Json(locations))
}

#[rocket::get("/anon/organisation_list")]
pub fn organisation_list() -> Result<Json<Vec<Organisation>>, Error> {
    let organisations = crate::db::organisation::organisation_list()?;
    Ok(Json(organisations))
}

#[rocket::get("/anon/skill_list")]
pub fn skill_list() -> Result<Json<Vec<Skill>>, Error> {
    let skills = crate::db::skill::skill_list()?;
    Ok(Json(skills))
}

#[rocket::get("/anon/club_list")]
pub fn club_list() -> Result<Json<Vec<Club>>, Error> {
    let clubs = crate::db::club::club_list()?;
    Ok(Json(clubs))
}

#[rocket::get("/anon/course_list")]
pub fn course_list() -> Result<Json<Vec<Course>>, Error> {
    let courses = crate::db::course::course_list(None, Some(true), Some(true))?;
    Ok(Json(courses))
}

#[rocket::get("/user_salt?<user_key>")]
pub fn user_salt(user_key: String) -> Result<String, Error> {
    let salt = crate::db::user::user_salt_value(&user_key);

    // If the user does not exist, just return a random salt to prevent data scraping
    match salt {
        Err(_) => Ok(hex::encode(crate::common::hash128_string(&user_key))),
        Ok(salt) => Ok(hex::encode(salt)),
    }
}
