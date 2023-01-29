use rocket::http::Status;
use rocket::serde::json::Json;

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::api::ApiError;
use crate::db::get_pool_conn;
use crate::session::{SlotSession};
use crate::common::{Slot, User};

/*
 * ROUTES
 */

#[rocket::get("/slot/slot_info")]
pub fn slot_info(session: SlotSession) -> Result<Json<Slot>, Status> {
    match crate::db_slot::get_slot_info(session.slot_id) {
        None => Err(Status::InternalServerError),
        Some(slot) => Ok(Json(slot)),
    }
}

#[rocket::get("/slot/slot_candidate_list")]
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

#[rocket::get("/slot/slot_participant_list")]
pub fn slot_participant_list(session: SlotSession) -> Result<Json<Vec<User>>, ApiError> {
    match crate::db_slot::list_slot_participants(session.slot_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(users) => Ok(Json(users)),
    }
}

#[rocket::head("/slot/slot_participant_add?<user_id>")]
pub fn slot_participant_add(user_id: i64, session: SlotSession) -> Result<(), ApiError> {
    match crate::db_slot::add_slot_participant(session.slot_id, user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(()) => Ok(()),
    }
}

#[rocket::head("/slot/slot_participant_remove?<user_id>")]
pub fn slot_participant_remove(user_id: i64, session: SlotSession) -> Result<(), ApiError> {
    match crate::db_slot::remove_slot_participant(session.slot_id, user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(()) => Ok(()),
    }
}
