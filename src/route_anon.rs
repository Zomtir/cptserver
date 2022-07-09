extern crate lazy_static;

use rocket::serde::json::Json;
use rocket::http::Status;

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::db::get_pool_conn;
use crate::common::{Location, Branch, Access};

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

#[rocket::get("/access_list")]
pub fn access_list() -> Result<Json<Vec<Access>>,Status> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT access_id, access_key, title FROM access").unwrap();
    let map = |(access_id, access_key, title): (u8, String, String)| {
        Access {id: access_id, key: access_key, title}
    };

    match conn.exec_map(&stmt,params::Params::Empty,&map) {
        Err(..) => Err(Status::Conflict),
        Ok(accesses) => Ok(Json(accesses)),
    }
}