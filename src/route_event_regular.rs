use crate::clock::WebDate;
use crate::error::Error;
use rocket::serde::json::Json;

use crate::common::{Slot, SlotStatus};
use crate::session::UserSession;

/*
 * ROUTES
 */


#[rocket::get("/regular/event_list?<begin>&<end>&<status>&<location_id>")]
pub fn event_list(
    session: UserSession,
    begin: WebDate,
    end: WebDate,
    status: Option<SlotStatus>,
    location_id: Option<i64>) -> Result<Json<Vec<Slot>>, Error> {

    let frame_start = begin.to_naive();
    let frame_stop = end.to_naive();

    let window = frame_stop.signed_duration_since(frame_start);

    if window < crate::config::CONFIG_SLOT_LIST_TIME_MIN() || window > crate::config::CONFIG_SLOT_LIST_TIME_MAX() {
        return Err(Error::SlotWindowInvalid);
    }

    match crate::db_slot::list_slots(
        Some(frame_start),
        Some(frame_stop),
        status,
        location_id,
        None,
        Some(session.user.id),
    )? {
        slots => Ok(Json(slots)),
    }
}

#[rocket::post("/regular/event_create", format = "application/json", data = "<slot>")]
pub fn event_create(session: UserSession, mut slot: Json<Slot>) -> Result<String, Error> {
    crate::common::validate_slot_dates(&mut slot)?;

    let slot_id = crate::db_slot::slot_create(&mut slot, &"DRAFT", None)?;
    crate::db_slot::slot_owner_add(slot_id, session.user.id)?;
    Ok(slot_id.to_string())
}

#[rocket::get("/regular/event_owner_condition?<slot_id>")]
pub fn event_owner_condition(session: UserSession, slot_id: i64) -> Result<Json<bool>, Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        condition => Ok(Json(condition)),
    }
}
