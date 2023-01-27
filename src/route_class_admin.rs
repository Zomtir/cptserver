use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::common::{Slot};
use crate::session::UserSession;

#[rocket::get("/admin/class_list?<course_id>")]
pub fn class_list(session: UserSession, course_id: u32) -> Result<Json<Vec<Slot>>, ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    match crate::db_slot::list_slots(None, None, None, Some(course_id), None) {
        None => Err(ApiError::DB_CONFLICT),
        Some(slots) => Ok(Json(slots)),
    }
}

#[rocket::post(
    "/admin/class_create?<course_id>",
    format = "application/json",
    data = "<slot>"
)]
pub fn class_create(
    session: UserSession,
    course_id: u32,
    mut slot: Json<Slot>,
) -> Result<String, ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    crate::common::validate_slot_dates(&mut slot);

    match crate::db_slot::create_slot(&slot, "OCCURRING", &Some(course_id)) {
        None => Err(ApiError::DB_CONFLICT),
        Some(slot_id) => Ok(slot_id.to_string()),
    }
}

#[rocket::post(
    "/admin/class_edit?<slot_id>",
    format = "application/json",
    data = "<slot>"
)]
pub fn class_edit(
    session: UserSession,
    slot_id: i64,
    mut slot: Json<Slot>,
) -> Result<(), ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    crate::common::validate_slot_dates(&mut slot);

    match crate::db_slot::edit_slot(&slot_id, &slot) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(..) => (),
    };

    let password = match crate::common::validate_slot_password(&mut slot) {
        None => return Err(ApiError::SLOT_BAD_PASSWORD),
        Some(password) => password,
    };

    match crate::db_slot::edit_slot_password(slot_id, password) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/admin/class_delete?<slot_id>")]
pub fn class_delete(session: UserSession, slot_id: i64) -> Result<(), ApiError> {
    if !session.right.admin_courses {return Err(ApiError::RIGHT_NO_COURSES)};

    match crate::db_slot::delete_slot(slot_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(()) => Ok(()),
    }
}
