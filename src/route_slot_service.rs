use rocket::serde::json::Json;

use crate::common::{Slot, User};
use crate::error::Error;
use crate::session::SlotSession;

/*
 * ROUTES
 */

#[rocket::get("/service/slot_info")]
pub fn slot_info(session: SlotSession) -> Result<Json<Slot>, Error> {
    Ok(Json(crate::db_slot::slot_info(session.slot_id)?))
}

#[rocket::post("/service/slot_note_edit", format = "text/plain", data = "<note>")]
pub fn slot_note_edit(session: SlotSession, note: String) -> Result<(), Error> {
    crate::db_slot::slot_note_edit(session.slot_id, &note)?;
    Ok(())
}

#[rocket::get("/service/slot_participant_pool")]
pub fn slot_participant_pool(session: SlotSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db_slot::slot_participant_pool(session.slot_id)?;
    Ok(Json(users))
}

#[rocket::get("/service/slot_participant_list")]
pub fn slot_participant_list(session: SlotSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db_slot::slot_participant_list(session.slot_id)?;
    Ok(Json(users))
}

#[rocket::head("/service/slot_participant_add?<user_id>")]
pub fn slot_participant_add(user_id: u64, session: SlotSession) -> Result<(), Error> {
    crate::db_slot::slot_participant_add(session.slot_id, user_id)
}

#[rocket::head("/service/slot_participant_remove?<user_id>")]
pub fn slot_participant_remove(user_id: u64, session: SlotSession) -> Result<(), Error> {
    crate::db_slot::slot_participant_remove(session.slot_id, user_id)
}

#[rocket::get("/service/slot_owner_pool")]
pub fn slot_owner_pool(session: SlotSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db_slot::slot_owner_pool(session.slot_id)?;
    Ok(Json(users))
}

#[rocket::get("/service/slot_owner_list")]
pub fn slot_owner_list(session: SlotSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db_slot::slot_owner_list(session.slot_id)?;
    Ok(Json(users))
}

#[rocket::head("/service/slot_owner_add?<user_id>")]
pub fn slot_owner_add(user_id: u64, session: SlotSession) -> Result<(), Error> {
    crate::db_slot::slot_owner_add(session.slot_id, user_id)
}

#[rocket::head("/service/slot_owner_remove?<user_id>")]
pub fn slot_owner_remove(user_id: u64, session: SlotSession) -> Result<(), Error> {
    crate::db_slot::slot_owner_remove(session.slot_id, user_id)
}
