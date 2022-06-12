use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use rocket::http::Status;
use rocket_contrib::json::Json;

use crate::api::ApiError;
use crate::session::{POOL, UserSession, Slot, Location};

// TODO make a check that status is not an invalid string by implementing a proper trait
#[get("/reservation_list?<status>")]
pub fn reservation_list(status: String, session: UserSession) -> Result<Json<Vec<Slot>>,Status> {
    if !session.user.admin_reservations {return Err(Status::Unauthorized)};

    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("SELECT slot_id, slot_key, s.title, l.location_id, l.location_key, l.title, s.begin, s.end, s.status
                          FROM slots s
                          JOIN locations l ON l.location_id = s.location_id
                          WHERE status = :status").unwrap();

    let params = mysql::params! {
        "status" => &status,
    };
    let map = |(slot_id, slot_key, slot_title, location_id, location_key, location_title, begin, end, status): (u32, _, _, u32, _, _, _, _, String)| 
        Slot {
            id: slot_id, key: slot_key, pwd: None, title: slot_title, begin, end, status: Some(status),
            location: Location {id: location_id, key: location_key, title: location_title},
            course_id: None, user_id: Some(0)};
    // TODO user_id has to be added to the SQL request
    
    match conn.exec_map(&stmt,&params,&map) {
        Err(..) => Err(Status::InternalServerError),
        Ok(slots) => Ok(Json(slots)),
    }
}

#[head("/reservation_accept?<slot_id>")]
pub fn reservation_accept(slot_id: u32, session: UserSession) -> Result<Status,ApiError> {
    if !session.user.admin_reservations {return Err(ApiError::RIGHT_NO_RESERVATIONS)};

    // Perhaps lock the DB during checking and potentially accepting the request

    let slot : Slot = match crate::session::get_slot_info(&slot_id){
        None => return Err(ApiError::SLOT_NO_ENTRY),
        Some(slot) => slot,
    };

    // The check is here intentional to be able to return early although it is also checked during is_slot_free
    if !crate::session::is_slot_valid(&slot) {
        return Err(ApiError::SLOT_BAD_TIME);
    }

    let (status_update, response) = match crate::session::is_slot_free(&slot) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => ("REJECTED", Err(ApiError::SLOT_OVERLAP_TIME)),
        Some(true) => ("OCCURRING", Ok(Status::Ok)),
    };

    match crate::session::set_slot_status(slot.id, "PENDING", status_update) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => response,
    }
}

#[head("/reservation_deny?<slot_id>")]
pub fn reservation_deny(slot_id: u32, session: UserSession) -> Status {
    if !session.user.admin_reservations {return Status::Forbidden};

    match crate::session::set_slot_status(slot_id, "PENDING", "REJECTED") {
        None => Status::InternalServerError,
        Some(..) => Status::Ok,
    }
}

// TODO ruleset who is able to do this
#[head("/reservation_delete?<slot>")]
pub fn reservation_delete(slot: u32, session: UserSession) -> Status {
    if !session.user.admin_reservations {return Status::Forbidden};

    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("DELETE r FROM slots WHERE slot_id = :slot_id AND status = `OCCURRING`").unwrap();
    let params = mysql::params! {"slot_id" => slot};

    match conn.exec::<String,_,_>(&stmt,&params){
        Err(..) => Status::Conflict,
        Ok(..) => Status::Ok,
    }
}
