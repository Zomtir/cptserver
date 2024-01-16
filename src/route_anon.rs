extern crate lazy_static;

use rocket::http::Status;
use rocket::serde::json::Json;

use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Branch, Location, Course};
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

#[rocket::get("/branch_list")]
pub fn branch_list() -> Result<Json<Vec<Branch>>, Status> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT branch_id, branch_key, title FROM branches").unwrap();
    let map = |(branch_id, branch_key, title): (u16, String, String)| Branch {
        id: branch_id,
        key: branch_key,
        title,
    };

    match conn.exec_map(&stmt, params::Params::Empty, &map) {
        Err(..) => Err(Status::Conflict),
        Ok(branches) => Ok(Json(branches)),
    }
}

#[rocket::get("/user_salt?<user_key>")]
pub fn user_salt(user_key: String) -> Result<String, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT salt FROM users WHERE user_key = :user_key");
    let params = params! {
        "user_key" => &user_key
    };

    match conn.exec_first::<Vec<u8>, _, _>(&stmt.unwrap(), &params)? {
        None => Ok(hex::encode(crate::common::hash128_string(&user_key))),
        Some(salt) => Ok(hex::encode(salt)),
    }
}

#[rocket::get("/anon/course_list")]
pub fn course_list() -> Result<Json<Vec<Course>>, Error> {
    match crate::db_course::course_list(None, Some(true), Some(true))? {
        courses => Ok(Json(courses)),
    }
}
