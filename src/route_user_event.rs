use rocket::serde::json::Json;
use rocket::http::Status;
use crate::api::ApiError;

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::db::get_pool_conn;
use crate::session::UserSession;
use crate::common::{Slot, Location};
use crate::common::{random_string};

/*
 * ROUTES
 */

#[rocket::get("/indi_slot_list?<status>")]
pub fn indi_slot_list(session: UserSession, status: String) -> Result<Json<Vec<Slot>>,Status> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT s.slot_id, s.slot_key, s.title, l.location_id, l.location_key, l.title, s.begin, s.end, s.status
                          FROM slots s
                          JOIN locations l ON l.location_id = s.location_id
                          INNER JOIN slot_owners o ON s.slot_id = o.slot_id
                          WHERE o.user_id = :user_id AND s.status = :status").unwrap();

    let params = params! {
        "user_id" => session.user.id,
        "status" => &status,
    };
    let map = |(slot_id, slot_key, slot_title, location_id, location_key, location_title, begin, end, status): (u32, _, _, u32, _, _, _, _, String)| 
        Slot {
            id: slot_id, key: slot_key, title: slot_title, pwd: None, begin, end, status: Some(status),
            location: Location {id: location_id, key: location_key, title: location_title},
            course_id: None, owners: None};
    
    match conn.exec_map(&stmt,&params,&map) {
        Err(..) => return Err(Status::Conflict),
        Ok(slots) => return Ok(Json(slots)),
    };
}

#[rocket::post("/indi_slot_create", format = "application/json", data = "<slot>")]
pub fn indi_slot_create(session: UserSession, mut slot: Json<Slot>) -> Result<String, ApiError> {
    crate::db_slot::round_slot_window(&mut slot);

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO slots (slot_key, pwd, title, location_id, begin, end, status)
                          VALUES (:slot_key, :pwd, :title, :location_id, :begin, :end, :status)").unwrap();

    let params = params! {
        "slot_key" => random_string(8),
        "pwd" => random_string(8),
        "title" => &slot.title,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
        "status" => "DRAFT",
        "user_id" => &session.user.id,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => return Err(ApiError::DB_CONFLICT),
        Ok(..) => (),
    };
    
    let stmt_id = conn.prep("SELECT LAST_INSERT_ID()").unwrap();

    let slot_id = match conn.exec_first::<u32,_,_>(&stmt_id,params::Params::Empty) {
        Err(..) | Ok(None) => return Err(ApiError::DB_CONFLICT),
        Ok(Some(slot_id)) => slot_id,
    };

    match crate::db_slot::add_slot_owner(slot_id, session.user.id) {
        Err(e) => return Err(e),
        Ok(..) => (),
    };

    Ok(slot_id.to_string())
}

#[rocket::head("/event_owner_add?<slot_id>&<user_id>")]
pub fn event_owner_add(session: UserSession, slot_id: u32, user_id: u32) -> Result<Status,ApiError> {
    match crate::db_slot::is_slot_owner(&slot_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::SLOT_NO_OWNER),
        Some(true) => (),
    }

    match crate::db_slot::add_slot_owner(slot_id, user_id) {
        Err(e) => Err(e),
        Ok(..) => Ok(Status::Ok),
    }
}

#[rocket::head("/event_owner_remove?<slot_id>&<user_id>")]
pub fn event_owner_remove(session: UserSession, slot_id: u32, user_id: u32) -> Result<Status,ApiError> {
    match crate::db_slot::is_slot_owner(&slot_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::SLOT_NO_OWNER),
        Some(true) => (),
    }

    match crate::db_slot::remove_slot_owner(slot_id, user_id) {
        Err(e) => Err(e),
        Ok(..) => Ok(Status::Ok),
    }
}


// TODO, check times again... overall share more code with slot accept and slot_create
// TODO, allow inviting member for draft
// TODO, allow inviting groups for draft
#[rocket::post("/indi_slot_edit", format = "application/json", data = "<slot>")]
pub fn indi_slot_edit(session: UserSession, mut slot: Json<Slot>) -> Result<Status, ApiError> {
    crate::db_slot::round_slot_window(&mut slot);

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE slots SET
        title = :title,
        location_id = :location_id,
        begin = :begin,
        end = :end,
        status = 'DRAFT'
        WHERE slot_id = :slot_id AND user_id = :user_id
        AND (status = 'DRAFT' OR status = 'REJECTED' OR status = 'CANCELED')").unwrap();

    let params = params! {
        "slot_id" => &slot.id,
        "title" => &slot.title,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
        "user_id" => &session.user.id,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => return Err(ApiError::DB_CONFLICT),
        Ok(..) => (),
    };

    if slot.pwd.is_none() || slot.pwd.as_ref().unwrap().len() < 8 {
        return Err(ApiError::SLOT_BAD_PASSWORD);
    };

    let stmt_pwd = conn.prep("UPDATE slots SET pwd = :pwd WHERE slot_id = :slot_id AND user_id = :user_id").unwrap();
    let params_pwd = params! {
        "slot_id" => &slot.id,
        "pwd" => &slot.pwd.as_ref().unwrap(),
        "user_id" => &session.user.id,
    };

    match conn.exec_drop(&stmt_pwd,&params_pwd) {
        Err(..) => Err(ApiError::DB_CONFLICT),
        Ok(..) => Ok(Status::Ok),
    }
}

#[rocket::head("/indi_slot_submit?<slot_id>")]
pub fn indi_slot_submit(session: UserSession, slot_id: u32) -> Result<Status,ApiError> {
    // Perhaps lock the DB during checking and modifying the slot status

    // Check that user is responsible for this slot
    match crate::db_slot::is_slot_owner(&slot_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::RIGHT_CONFLICT),
        Some(true) => (),
    }

    let slot : Slot = match crate::db_slot::get_slot_info(&slot_id){
        None => return Err(ApiError::SLOT_NO_ENTRY),
        Some(slot) => slot,
    };

    // The check is here intentional to be able to return early although it is also checked during is_slot_free
    if !crate::db_slot::is_slot_valid(&slot) {
        return Err(ApiError::SLOT_BAD_TIME);
    }

    // Perhaps just leave the slot as draft if the time is not free
    let (status_update, response) = match crate::db_slot::is_slot_free(&slot) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => ("REJECTED", Err(ApiError::SLOT_OVERLAP_TIME)),
        Some(true) => match crate::config::CONFIG_RESERVATION_AUTO_ACCEPT {
            false => ("PENDING", Ok(Status::Ok)),
            true => ("OCCURRING", Ok(Status::Ok)),
        },
    };
    
    match crate::db_slot::set_slot_status(slot.id, "DRAFT", status_update) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => response,
    }
}

// TODO check that user is allowed to edit this slot
#[rocket::head("/indi_slot_withdraw?<slot_id>")]
pub fn indi_slot_withdraw(_session: UserSession, slot_id: u32) -> Status {
    match crate::db_slot::set_slot_status(slot_id, "PENDING", "DRAFT") {
        None => Status::InternalServerError,
        Some(..) => Status::Ok,
    }
}

// TODO check that user is allowed to edit this slot
#[rocket::head("/indi_slot_cancel?<slot_id>")]
pub fn indi_slot_cancel(_session: UserSession, slot_id: u32) -> Status {
    match crate::db_slot::set_slot_status(slot_id, "OCCURRING", "CANCELED") {
        None => Status::InternalServerError,
        Some(..) => Status::Ok,
    }
}

// TODO check that user is allowed to edit this slot
#[rocket::head("/indi_slot_recycle?<slot_id>")]
pub fn indi_slot_recycle(_session: UserSession, slot_id: u32) -> Status {
    match crate::db_slot::set_slot_status(slot_id, "REJECTED", "DRAFT") {
        None => Status::InternalServerError,
        Some(..) => Status::Ok,
    }
}

#[rocket::head("/indi_slot_delete?<slot_id>")]
pub fn indi_slot_delete(session: UserSession, slot_id: u32) -> Result<Status,ApiError> {
    // Perhaps lock the DB during checking and modifying the slot status
    match crate::db_slot::is_slot_owner(&slot_id, &session.user.id){
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::RIGHT_CONFLICT),
        Some(true) => (),
    };

    let slot : Slot = match crate::db_slot::get_slot_info(&slot_id){
        None => return Err(ApiError::SLOT_NO_ENTRY),
        Some(slot) => slot,
    };

    match slot.status {
        None => return Err(ApiError::DB_CONFLICT),
        Some(status) => match status.as_str() {
            "DRAFT" => (),
            _ => return Err(ApiError::SLOT_STATUS_INCOMPAT),
        },
    }

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE s FROM slots s
                          WHERE slot_id = :slot_id AND user_id = :user_id
                          AND status = 'DRAFT'").unwrap();

    let params = params! {
        "slot_id" => &slot_id,
        "user_id" => &session.user.id,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => Err(ApiError::DB_CONFLICT),
        Ok(..) => Ok(Status::Ok),
    }
}
