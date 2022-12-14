use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use rocket::http::Status;
use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::clock::WebDate;
use crate::common::{Location, Slot};
use crate::db::get_pool_conn;
use crate::session::UserSession;

// TODO make a check that status is not an invalid string by implementing a proper trait
// status shouldn't even be a searchable criteria? if so, please make it enum with FromFormValue::default()
// Default user_id should not be 0, but NULL
#[rocket::get("/reservation_list?<begin>&<end>&<status>&<user_id>")]
pub fn reservation_list(
    session: UserSession,
    begin: WebDate,
    end: WebDate,
    status: Option<String>,
    user_id: Option<u32>,
) -> Result<Json<Vec<Slot>>, ApiError> {
    if !session.user.admin_reservations {
        return Err(ApiError::RIGHT_NO_RESERVATIONS);
    };

    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT s.slot_id, s.slot_key, s.title, l.location_id, l.location_key, l.title, s.begin, s.end, s.status
                          FROM slots s
                          JOIN locations l ON l.location_id = s.location_id
                          INNER JOIN slot_owners o ON s.slot_id = o.slot_id
                          WHERE s.begin > :frame_start
                          AND s.begin < :frame_stop
                          AND (('' = :status) OR (s.status = :status))
                          AND (('0' = :user_id) OR (o.user_id = :user_id))").unwrap();

    let frame_start = begin.to_naive();
    let frame_stop = end.to_naive();

    let window = frame_stop.signed_duration_since(frame_start).num_days();

    if window < 1 || window > 366 {
        return Err(ApiError::INVALID_RANGE);
    }

    let params = params! {
        "frame_start" => &frame_start,
        "frame_stop" => &frame_stop,
        "status" => &status,
        "user_id" => &user_id,
    };

    let map = |(
        slot_id,
        slot_key,
        slot_title,
        location_id,
        location_key,
        location_title,
        begin,
        end,
        status,
    ): (u32, String, String, u32, _, _, _, _, String)| Slot {
        id: slot_id,
        key: slot_key,
        pwd: None,
        title: slot_title,
        begin,
        end,
        status: Some(status),
        location: Location {
            id: location_id,
            key: location_key,
            title: location_title,
        },
        course_id: None,
        owners: None,
    };

    match conn.exec_map(&stmt, &params, &map) {
        Err(..) => return Err(ApiError::DB_CONFLICT),
        Ok(slots) => return Ok(Json(slots)),
    };
}

#[rocket::head("/reservation_accept?<slot_id>")]
pub fn reservation_accept(session: UserSession, slot_id: u32) -> Result<Status, ApiError> {
    if !session.user.admin_reservations {
        return Err(ApiError::RIGHT_NO_RESERVATIONS);
    };

    // Perhaps lock the DB during checking and potentially accepting the request

    let slot: Slot = match crate::db_slot::get_slot_info(&slot_id) {
        None => return Err(ApiError::SLOT_NO_ENTRY),
        Some(slot) => slot,
    };

    // The check is here intentional to be able to return early although it is also checked during is_slot_free
    if !crate::db_slot::is_slot_valid(&slot) {
        return Err(ApiError::SLOT_BAD_TIME);
    }

    let (status_update, response) = match crate::db_slot::is_slot_free(&slot) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => ("REJECTED", Err(ApiError::SLOT_OVERLAP_TIME)),
        Some(true) => ("OCCURRING", Ok(Status::Ok)),
    };

    match crate::db_slot::set_slot_status(slot.id, "PENDING", status_update) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => response,
    }
}

#[rocket::head("/reservation_deny?<slot_id>")]
pub fn reservation_deny(session: UserSession, slot_id: u32) -> Status {
    if !session.user.admin_reservations {
        return Status::Forbidden;
    };

    match crate::db_slot::set_slot_status(slot_id, "PENDING", "REJECTED") {
        None => Status::InternalServerError,
        Some(..) => Status::Ok,
    }
}

#[rocket::head("/reservation_cancel?<slot_id>")]
pub fn reservation_cancel(session: UserSession, slot_id: u32) -> Status {
    if !session.user.admin_reservations {
        return Status::Forbidden;
    };

    match crate::db_slot::set_slot_status(slot_id, "OCCURRING", "REJECTED") {
        None => Status::InternalServerError,
        Some(..) => Status::Ok,
    }
}
