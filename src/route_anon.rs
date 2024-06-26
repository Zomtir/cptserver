extern crate lazy_static;

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::common::{Club, Course, Location, Skill};

use crate::error::Error;

#[rocket::head("/status")]
pub fn status() -> Status {
    Status::Ok
}

#[rocket::get("/anon/location_list")]
pub fn location_list() -> Result<Json<Vec<Location>>, Error> {
    let locations = crate::db_location::location_list()?;
    Ok(Json(locations))
}

#[rocket::get("/anon/skill_list")]
pub fn skill_list() -> Result<Json<Vec<Skill>>, Error> {
    let skills = crate::db_skill::skill_list()?;
    Ok(Json(skills))
}

#[rocket::get("/anon/club_list")]
pub fn club_list() -> Result<Json<Vec<Club>>, Error> {
    let clubs = crate::db_club::club_list()?;
    Ok(Json(clubs))
}

#[rocket::get("/anon/course_list")]
pub fn course_list() -> Result<Json<Vec<Course>>, Error> {
    let courses = crate::db_course::course_list(None, Some(true), Some(true))?;
    Ok(Json(courses))
}

#[rocket::get("/user_salt?<user_key>")]
pub fn user_salt(user_key: String) -> Result<String, Error> {
    let salt = crate::db_user::user_salt_value(&user_key);

    // If the user does not exist, just return a random salt to prevent data scraping
    match salt {
        Err(_) => Ok(hex::encode(crate::common::hash128_string(&user_key))),
        Ok(salt) => Ok(hex::encode(salt)),
    }
}
