use rocket::serde::json::Json;

use crate::common::{Slot, WebDate};
use crate::error::Error;
use crate::session::UserSession;

/*
 * ROUTES
 */
#[rocket::get("/regular/slot_list?<begin>&<end>")]
pub fn slot_list(_session: UserSession, begin: WebDate, end: WebDate) -> Result<Json<Vec<Slot>>, Error> {
    let slots = crate::db_slot::list_slots(
        Some(begin.to_naive()),
        Some(end.to_naive()),
        None,
        None,
        None,
        None,
        None,
    )?;
    Ok(Json(slots))
}
