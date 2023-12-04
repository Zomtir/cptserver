use rocket::serde::json::Json;

use crate::clock::WebDate;
use crate::common::Slot;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/event_list?<begin>&<end>&<status>&<location_id>&<owner_id>")]
pub fn event_list(
    session: UserSession,
    begin: WebDate,
    end: WebDate,
    status: Option<String>,
    location_id: Option<i64>,
    owner_id: Option<i64>,
) -> Result<Json<Vec<Slot>>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

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
        owner_id)? {
        slots => Ok(Json(slots)),
    }
}

#[rocket::head("/admin/event_accept?<slot_id>")]
pub fn event_accept(session: UserSession, slot_id: i64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    // Perhaps lock the DB during checking and potentially accepting the request
    let slot: Slot = crate::db_slot::get_slot_info(slot_id)?;

    // The check is here intentional to be able to return early although it is also checked during is_slot_free
    if !crate::common::is_slot_valid(&slot) {
        return Err(Error::SlotWindowInvalid);
    }

    let status_update = match crate::db_slot::is_slot_free(&slot)? {
        false => "REJECTED",
        true => "OCCURRING",
    };

    crate::db_slot::edit_slot_status(slot.id, "PENDING", status_update)?;
    Ok(())
}

#[rocket::head("/admin/event_deny?<slot_id>")]
pub fn event_deny(session: UserSession, slot_id: i64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::edit_slot_status(slot_id, "PENDING", "REJECTED")?;
    Ok(())
}

#[rocket::head("/admin/event_cancel?<slot_id>")]
pub fn event_cancel(session: UserSession, slot_id: i64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::edit_slot_status(slot_id, "OCCURRING", "REJECTED")?;
    Ok(())
}
