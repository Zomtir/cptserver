use rocket::serde::json::Json;
use crate::api::ApiError;

use crate::session::UserSession;
use crate::common::{Slot};

/*
 * ROUTES
 */

#[rocket::get("/member/event_list?<status>")]
pub fn event_list(session: UserSession, status: Option<String>) -> Result<Json<Vec<Slot>>,ApiError> {
    let begin = chrono::Utc::now().naive_utc() - chrono::Duration::days(30);
    let end = chrono::Utc::now().naive_utc() + chrono::Duration::days(90);

    match crate::db_slot::list_slots(Some(begin.date()), Some(end.date()), status, None, Some(session.user.id)) {
        None => Err(ApiError::DB_CONFLICT),
        Some(slots) => Ok(Json(slots)),
    }
}

#[rocket::post("/member/event_create", format = "application/json", data = "<slot>")]
pub fn event_create(session: UserSession, mut slot: Json<Slot>) -> Result<String, ApiError> {
    crate::common::validate_slot_dates(&mut slot);

    let slot_id = match crate::db_slot::create_slot(&mut slot, &"DRAFT", &None) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(slot_id) => slot_id,
    };

    match crate::db_slot::add_slot_owner(slot.id, session.user.id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(()) => Ok(slot_id.to_string()),
    }
}
