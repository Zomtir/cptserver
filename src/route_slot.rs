use rocket::serde::json::Json;

use crate::common::{Slot, User};
use crate::error::Error;
use crate::session::SlotSession;

/*
 * ROUTES
 */

#[rocket::get("/slot/slot_info")]
pub fn slot_info(session: SlotSession) -> Result<Json<Slot>, Error> {
    Ok(Json(crate::db_slot::get_slot_info(session.slot_id)?))
}

#[rocket::get("/slot/slot_participant_pool")]
pub fn slot_candidates(session: SlotSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db_slot::slot_participant_pool(session.slot_id)?;
    Ok(Json(users))
}

#[rocket::get("/slot/slot_participant_list")]
pub fn slot_participant_list(session: SlotSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db_slot::slot_participant_list(session.slot_id)?;
    Ok(Json(users))
}

#[rocket::head("/slot/slot_participant_add?<user_id>")]
pub fn slot_participant_add(user_id: i64, session: SlotSession) -> Result<(), Error> {
    crate::db_slot::slot_participant_add(session.slot_id, user_id)
}

#[rocket::head("/slot/slot_participant_remove?<user_id>")]
pub fn slot_participant_remove(user_id: i64, session: SlotSession) -> Result<(), Error> {
    crate::db_slot::slot_participant_remove(session.slot_id, user_id)
}
