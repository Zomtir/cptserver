extern crate lazy_static;

use rocket::serde::json::Json;
use rocket::http::Status;

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::api::ApiError;
use crate::db::get_pool_conn;
use crate::common::{Location, Branch};

#[rocket::head("/status")]
pub fn status() -> Status {    
    Status::Ok
}

#[rocket::get("/location_list")]
pub fn location_list() -> Result<Json<Vec<Location>>, Status> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT location_id, location_key, title FROM locations").unwrap();
    let map = |(location_id, location_key, title): (u32, _, _,)| {
        Location {id: location_id, key: location_key, title}
    };

    match conn.exec_map(&stmt,params::Params::Empty,&map) {
        Err(..) => Err(Status::Conflict),
        Ok(locations) => Ok(Json(locations)),
    }
}

#[rocket::get("/branch_list")]
pub fn branch_list() -> Result<Json<Vec<Branch>>,Status> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT branch_id, branch_key, title FROM branches").unwrap();
    let map = |(branch_id, branch_key, title): (u16, String, String)| {
        Branch {id: branch_id, key: branch_key, title}
    };

    match conn.exec_map(&stmt,params::Params::Empty,&map) {
        Err(..) => Err(Status::Conflict),
        Ok(branches) => Ok(Json(branches)),
    }
}

#[rocket::get("/user_salt?<user_key>")]
pub fn user_salt(user_key: String) -> Result<String, ApiError> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT salt FROM users WHERE user_key = :user_key");
    let params = params! {
        "user_key" => &user_key
    };

    match conn.exec_first::<Vec<u8>, _, _>(&stmt.unwrap(), &params) {
        Err(..) => Err(ApiError::DB_CONFLICT),
        Ok(None) => Ok(hex::encode(crate::common::hash128_string(&user_key))),
        Ok(Some(salt)) => Ok(hex::encode(salt)),
    }
}