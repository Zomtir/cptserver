use crate::error::Error;
use rocket::serde::json::Json;

use crate::common::Slot;
use crate::session::UserSession;

/*
 * ROUTES
 */

#[rocket::get("/member/event_list?<status>")]
pub fn event_list(session: UserSession, status: Option<String>) -> Result<Json<Vec<Slot>>, Error> {
    let begin = chrono::Utc::now().naive_utc() - chrono::Duration::days(30);
    let end = chrono::Utc::now().naive_utc() + chrono::Duration::days(90);

    match crate::db_slot::list_slots(
        Some(begin.date()),
        Some(end.date()),
        status,
        None,
        Some(session.user.id),
    )? {
        slots => Ok(Json(slots)),
    }
}

#[rocket::post("/member/event_create", format = "application/json", data = "<slot>")]
pub fn event_create(session: UserSession, mut slot: Json<Slot>) -> Result<String, Error> {
    crate::common::validate_slot_dates(&mut slot)?;

    let slot_id = crate::db_slot::create_slot(&mut slot, &"DRAFT", None)?;
    crate::db_slot::slot_owner_add(slot.id, session.user.id)?;
    Ok(slot_id.to_string())
}

#[rocket::head("/member/event_owner_condition?<slot_id>")]
pub fn event_owner_condition(session: UserSession, slot_id: i64) -> Result<String, Error> {
    match crate::db_slot::is_slot_owner(slot_id, session.user.id)? {
        condition => Ok(condition.to_string()),
    }
}
