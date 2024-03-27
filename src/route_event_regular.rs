use crate::common::{Slot, SlotStatus, WebDateTime};
use crate::error::Error;
use crate::session::UserSession;
use rocket::serde::json::Json;

/*
 * ROUTES
 */

#[rocket::get("/regular/event_list?<begin>&<end>&<status>&<location_id>&<course_true>&<course_id>")]
pub fn event_list(
    session: UserSession,
    begin: Option<WebDateTime>,
    end: Option<WebDateTime>,
    location_id: Option<u64>,
    status: Option<SlotStatus>,
    course_true: Option<bool>,
    course_id: Option<u64>,
) -> Result<Json<Vec<Slot>>, Error> {
    let slots = crate::db_slot::list_slots(
        begin.map(|dt| dt.to_naive()),
        end.map(|dt| dt.to_naive()),
        status,
        location_id,
        course_true,
        course_id,
        Some(session.user.id),
    )?;
    Ok(Json(slots))
}

#[rocket::post("/regular/event_create", format = "application/json", data = "<slot>")]
pub fn event_create(session: UserSession, mut slot: Json<Slot>) -> Result<String, Error> {
    crate::common::validate_slot_dates(&mut slot)?;

    let slot_id = crate::db_slot::slot_create(&slot, "DRAFT", None)?;
    crate::db_slot::slot_owner_add(slot_id, session.user.id)?;
    Ok(slot_id.to_string())
}

#[rocket::get("/regular/event_owner_true?<slot_id>")]
pub fn event_owner_true(session: UserSession, slot_id: u64) -> Result<Json<bool>, Error> {
    let condition = crate::db_slot::slot_owner_true(slot_id, session.user.id)?;
    Ok(Json(condition))
}

#[rocket::head("/regular/event_participant_add?<slot_id>")]
pub fn event_participant_add(session: UserSession, slot_id: u64) -> Result<(), Error> {
    // TODO check if you can participate

    crate::db_slot::slot_participant_add(slot_id, session.user.id)?;
    Ok(())
}

#[rocket::head("/regular/event_participant_remove?<slot_id>")]
pub fn event_participant_remove(session: UserSession, slot_id: u64) -> Result<(), Error> {
    // TODO check if you can participate

    crate::db_slot::slot_participant_remove(slot_id, session.user.id)?;
    Ok(())
}
