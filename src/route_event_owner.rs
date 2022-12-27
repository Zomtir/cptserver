use rocket::serde::json::Json;
use crate::api::ApiError;

use crate::session::UserSession;
use crate::common::{Slot, User};

/*
 * ROUTES
 */

// TODO, check times again... overall share more code with slot accept and slot_create
// TODO, allow inviting member for draft
// TODO, allow inviting groups for draft
#[rocket::post("/owner/event_edit", format = "application/json", data = "<slot>")]
pub fn event_edit(session: UserSession, mut slot: Json<Slot>) -> Result<(), ApiError> {
    match crate::db_slot::is_slot_owner(&slot.id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::RIGHT_CONFLICT),
        Some(true) => (),
    }

    crate::common::validate_slot_dates(&mut slot);

    match crate::db_event::edit_event(&slot) {
        None => Err(ApiError::DB_CONFLICT),
        Some(()) => Ok(()),
    }
}

#[rocket::head("/owner/event_submit?<slot_id>")]
pub fn event_submit(session: UserSession, slot_id: i64) -> Result<(),ApiError> {
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
    if !crate::common::is_slot_valid(&slot) {
        return Err(ApiError::SLOT_BAD_TIME);
    }

    let is_free : bool = match crate::db_slot::is_slot_free(&slot) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(is_free) => is_free,
    };

    let status_update = match crate::config::CONFIG_RESERVATION_AUTO_CHECK {
        false => "PENDING",
        true => match is_free {
            true => "OCCURRING",
            false => "REJECTED",
        },
    };
    
    match crate::db_slot::set_slot_status(slot.id, "DRAFT", status_update) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/owner/event_withdraw?<slot_id>")]
pub fn event_withdraw(session: UserSession, slot_id: i64) -> Result<(),ApiError> {
    match crate::db_slot::is_slot_owner(&slot_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::RIGHT_CONFLICT),
        Some(true) => (),
    }

    match crate::db_slot::set_slot_status(slot_id, "PENDING", "DRAFT") {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/owner/event_cancel?<slot_id>")]
pub fn event_cancel(session: UserSession, slot_id: i64) -> Result<(),ApiError> {
    match crate::db_slot::is_slot_owner(&slot_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::RIGHT_CONFLICT),
        Some(true) => (),
    }

    match crate::db_slot::set_slot_status(slot_id, "OCCURRING", "CANCELED") {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/owner/event_recycle?<slot_id>")]
pub fn event_recycle(session: UserSession, slot_id: i64) -> Result<(),ApiError> {
    match crate::db_slot::is_slot_owner(&slot_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::RIGHT_CONFLICT),
        Some(true) => (),
    }

    match crate::db_slot::set_slot_status(slot_id, "REJECTED", "DRAFT") {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/owner/event_delete?<slot_id>")]
pub fn event_delete(session: UserSession, slot_id: i64) -> Result<(),ApiError> {
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

    match crate::db_event::delete_event(slot.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(()) => Ok(()),
    }
}


#[rocket::get("/owner/event_owner_list?<slot_id>")]
pub fn event_owner_list(session: UserSession, slot_id: i64) -> Result<Json<Vec<User>>,ApiError> {
    match crate::db_slot::is_slot_owner(&slot_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::SLOT_NO_OWNER),
        Some(true) => (),
    };

    match crate::db_slot::get_slot_owners(&slot_id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(users) => Ok(Json(users)),
    }
}

#[rocket::head("/owner/event_owner_add?<slot_id>&<user_id>")]
pub fn event_owner_add(session: UserSession, slot_id: i64, user_id: u32) -> Result<(),ApiError> {
    match crate::db_slot::is_slot_owner(&slot_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::SLOT_NO_OWNER),
        Some(true) => (),
    }

    match crate::db_slot::add_slot_owner(slot_id, user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(()) => Ok(()),
    }
}

#[rocket::head("/owner/event_owner_remove?<slot_id>&<user_id>")]
pub fn event_owner_remove(session: UserSession, slot_id: i64, user_id: u32) -> Result<(),ApiError> {
    match crate::db_slot::is_slot_owner(&slot_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::SLOT_NO_OWNER),
        Some(true) => (),
    }

    match crate::db_slot::remove_slot_owner(slot_id, user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(()) => Ok(()),
    }
}