extern crate lazy_static;

use rocket::http::Status;
use rocket::serde::json::Json;

use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Club, Course, Location, Skill};
use crate::db::get_pool_conn;
use crate::error::Error;

#[rocket::head("/status")]
pub fn status() -> Status {
    Status::Ok
}

#[rocket::get("/location_list")]
pub fn location_list() -> Result<Json<Vec<Location>>, Status> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn
        .prep("SELECT location_id, location_key, title FROM locations")
        .unwrap();
    let map = |(location_id, location_key, title): (u32, _, _)| Location {
        id: location_id,
        key: location_key,
        title,
    };

    match conn.exec_map(&stmt, params::Params::Empty, &map) {
        Err(..) => Err(Status::Conflict),
        Ok(locations) => Ok(Json(locations)),
    }
}

#[rocket::get("/skill_list")]
pub fn skill_list() -> Result<Json<Vec<Skill>>, Status> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT skill_id, skill_key, title FROM skills").unwrap();
    let map = |(skill_id, skill_key, title): (u16, String, String)| Skill {
        id: skill_id,
        key: skill_key,
        title,
    };

    match conn.exec_map(&stmt, params::Params::Empty, &map) {
        Err(..) => Err(Status::Conflict),
        Ok(skills) => Ok(Json(skills)),
    }
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
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT salt FROM users WHERE user_key = :user_key")?;
    let params = params! {
        "user_key" => &user_key
    };

    // If the user does not exist, just return a random salt to prevent data scraping
    match conn.exec_first::<Vec<u8>, _, _>(&stmt, &params)? {
        None => Ok(hex::encode(crate::common::hash128_string(&user_key))),
        Some(salt) => Ok(hex::encode(salt)),
    }
}
