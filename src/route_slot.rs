use rocket::http::Status;
use rocket::serde::json::Json;

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::db::get_pool_conn;
use crate::session::{SlotSession};
use crate::common::{Slot, User};

/*
 * ROUTES
 */

#[rocket::get("/slot_info")]
pub fn slot_info(session: SlotSession) -> Result<Json<Slot>, Status> {
    match crate::db_slot::get_slot_info(&session.slot_id) {
        None => Err(Status::InternalServerError),
        Some(slot) => Ok(Json(slot)),
    }
}

#[rocket::get("/slot_candidates")]
pub fn slot_candidates(_session: SlotSession) -> Result<Json<Vec<User>>,Status> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT user_id, user_key, firstname, lastname FROM users
                          WHERE enabled = TRUE").unwrap();

    let map = |(user_id, user_key, firstname, lastname)|
        User::from_info(user_id, user_key, firstname, lastname);

    // TODO level check threshold if existent

    match conn.exec_map(&stmt,params::Params::Empty,&map) {
        Err(..) => Err(Status::InternalServerError),
        Ok(users) => Ok(Json(users)),
    }
}

#[rocket::get("/slot_participants")]
pub fn slot_participants(session: SlotSession) -> Result<Json<Vec<User>>, Status> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT u.user_id, u.user_key, u.firstname, u.lastname
                          FROM slot_enrollments e JOIN users u ON (e.user_id = u.user_id)
                          WHERE slot_id = :slot_id").unwrap();
    let params = params! { "slot_id" => session.slot_id };
    let map = |(user_id, user_key, firstname, lastname)| {
        User::from_info(user_id, user_key, firstname, lastname)
    };

    match conn.exec_map(&stmt,&params,&map) {
        Err(..) => Err(Status::InternalServerError),
        Ok(users) => Ok(Json(users)),
    }
}

#[rocket::head("/slot_enrol?<user_id>")]
pub fn slot_enrol(user_id: u32, session: SlotSession) -> Status {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO slot_enrollments (slot_id, user_id)
                          SELECT :slot_id, user_id FROM users
                          WHERE user_id = :user_id").unwrap();
    let params = params! {
        "slot_id" => session.slot_id,
        "user_id" => &user_id,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => Status::InternalServerError,
        Ok(..) => Status::Ok,
    }
}

#[rocket::head("/slot_dismiss?<user_id>")]
pub fn slot_dismiss(user_id: u32, session: SlotSession) -> Status {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE e FROM slot_enrollments e
                          JOIN users u ON (e.user_id = u.user_id)
                          WHERE e.slot_id = :slot_id AND e.user_id = :user_id").unwrap();
    let params = params! {
        "slot_id" => session.slot_id,
        "user_id" => &user_id,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => Status::InternalServerError,
        Ok(..) => Status::Ok,
    }
}
