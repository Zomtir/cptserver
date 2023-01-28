use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::clock::WebDate;
use crate::common::{Slot};
use crate::session::UserSession;

#[rocket::get("/admin/event_list?<begin>&<end>&<status>&<owner_id>")]
pub fn event_list(
    session: UserSession,
    begin: WebDate,
    end: WebDate,
    status: Option<String>,
    owner_id: Option<u32>,
) -> Result<Json<Vec<Slot>>, ApiError> {
    if !session.right.admin_event {
        return Err(ApiError::RIGHT_NO_EVENT);
    };

    let frame_start = begin.to_naive();
    let frame_stop = end.to_naive();

    let window = frame_stop.signed_duration_since(frame_start).num_days();

    if window < crate::config::CONFIG_SLOT_WINDOW_DAY_MIN || window > crate::config::CONFIG_SLOT_WINDOW_DAY_MAX {
        return Err(ApiError::INVALID_RANGE);
    }

    match crate::db_slot::list_slots(Some(frame_start), Some(frame_stop), status, None, owner_id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(slots) => return Ok(Json(slots)),
    };
}

#[rocket::head("/admin/event_accept?<slot_id>")]
pub fn event_accept(session: UserSession, slot_id: i64) -> Result<(), ApiError> {
    if !session.right.admin_event {
        return Err(ApiError::RIGHT_NO_EVENT);
    };

    // Perhaps lock the DB during checking and potentially accepting the request

    let slot: Slot = match crate::db_slot::get_slot_info(slot_id) {
        None => return Err(ApiError::SLOT_NO_ENTRY),
        Some(slot) => slot,
    };

    // The check is here intentional to be able to return early although it is also checked during is_slot_free
    if !crate::common::is_slot_valid(&slot) {
        return Err(ApiError::SLOT_BAD_TIME);
    }

    let status_update = match crate::db_slot::is_slot_free(&slot) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => "REJECTED",
        Some(true) => "OCCURRING",
    };

    match crate::db_slot::edit_slot_status(slot.id, "PENDING", status_update) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/admin/event_deny?<slot_id>")]
pub fn event_deny(session: UserSession, slot_id: i64) -> Result<(), ApiError> {
    if !session.right.admin_event {
        return Err(ApiError::RIGHT_NO_EVENT);
    };

    match crate::db_slot::edit_slot_status(slot_id, "PENDING", "REJECTED") {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/admin/event_cancel?<slot_id>")]
pub fn event_cancel(session: UserSession, slot_id: i64) -> Result<(), ApiError> {
    if !session.right.admin_event {
        return Err(ApiError::RIGHT_NO_EVENT);
    };

    match crate::db_slot::edit_slot_status(slot_id, "OCCURRING", "REJECTED") {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}
