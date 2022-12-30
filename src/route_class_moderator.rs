use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::common::{Slot};
use crate::session::UserSession;

#[rocket::get("/mod/class_list?<course_id>")]
pub fn class_list(session: UserSession, course_id: u32) -> Result<Json<Vec<Slot>>, ApiError> {
    match crate::db_course::is_course_moderator(&course_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::COURSE_NO_MODERATOR),
        Some(true) => (),
    };

    match crate::db_slot::list_slots(None, None, None, Some(course_id), None) {
        None => Err(ApiError::DB_CONFLICT),
        Some(slots) => Ok(Json(slots)),
    }
}

#[rocket::post(
    "/mod/class_create?<course_id>",
    format = "application/json",
    data = "<slot>"
)]
pub fn class_create(
    session: UserSession,
    course_id: u32,
    mut slot: Json<Slot>,
) -> Result<String, ApiError> {
    match crate::db_course::is_course_moderator(&course_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::COURSE_NO_MODERATOR),
        Some(true) => (),
    };

    crate::common::validate_slot_dates(&mut slot);

    match crate::db_slot::create_slot(&slot, "OCCURRING", &Some(course_id)) {
        None => Err(ApiError::DB_CONFLICT),
        Some(slot_id) => Ok(slot_id.to_string()),
    }
}

#[rocket::post(
    "/mod/class_edit?<slot_id>",
    format = "application/json",
    data = "<slot>"
)]
pub fn class_edit(
    session: UserSession,
    slot_id: i64,
    mut slot: Json<Slot>,
) -> Result<(), ApiError> {
    match crate::db_slot::is_slot_moderator(&slot_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::COURSE_NO_MODERATOR),
        Some(true) => (),
    };

    crate::common::validate_slot_dates(&mut slot);

    match crate::db_slot::edit_slot(&slot_id, &slot) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(..) => (),
    };

    if slot.pwd.is_none() || slot.pwd.as_ref().unwrap().len() < 8 {
        return Err(ApiError::SLOT_BAD_PASSWORD);
    };

    match crate::db_slot::edit_slot_password(&slot_id, &slot.pwd.as_ref().unwrap()) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/mod/class_delete?<slot_id>")]
pub fn class_delete(session: UserSession, slot_id: i64) -> Result<(), ApiError> {
    match crate::db_slot::is_slot_moderator(&slot_id, &session.user.id) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => return Err(ApiError::COURSE_NO_MODERATOR),
        Some(true) => (),
    };

    match crate::db_slot::delete_slot(&slot_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(()) => Ok(()),
    }
}
