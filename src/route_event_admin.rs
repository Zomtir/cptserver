use rocket::serde::json::Json;

use crate::common::{Slot, SlotStatus, WebDate};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/event_list?<begin>&<end>&<status>&<location_id>&<owner_id>")]
pub fn event_list(
    session: UserSession,
    begin: WebDate,
    end: WebDate,
    status: Option<SlotStatus>,
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

    let slots = crate::db_slot::list_slots(
        Some(frame_start),
        Some(frame_stop),
        status,
        location_id,
        Some(false),
        None,
        owner_id,
    )?;
    Ok(Json(slots))
}

#[rocket::get("/admin/event_info?<slot_id>")]
pub fn event_info(session: UserSession, slot_id: i64) -> Result<Json<Slot>, Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    Ok(Json(crate::db_slot::slot_info(slot_id)?))
}

#[rocket::post("/admin/event_edit?<slot_id>", format = "application/json", data = "<slot>")]
pub fn event_edit(session: UserSession, slot_id: i64, mut slot: Json<Slot>) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::common::validate_slot_dates(&mut slot)?;

    crate::db_slot::edit_slot(slot_id, &slot)?;
    Ok(())
}

#[rocket::post("/admin/event_edit_password?<slot_id>", format = "text/plain", data = "<password>")]
pub fn event_edit_password(session: UserSession, slot_id: i64, password: String) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::edit_slot_password(slot_id, password)?;
    Ok(())
}

#[rocket::head("/admin/event_delete?<slot_id>")]
pub fn event_delete(session: UserSession, slot_id: i64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::slot_delete(slot_id)?;
    Ok(())
}

#[rocket::head("/admin/event_accept?<slot_id>")]
pub fn event_accept(session: UserSession, slot_id: i64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    // Perhaps lock the DB during checking and potentially accepting the request
    let slot: Slot = crate::db_slot::slot_info(slot_id)?;

    // The check is here intentional to be able to return early although it is also checked during is_slot_free
    if !crate::common::is_slot_valid(&slot) {
        return Err(Error::SlotWindowInvalid);
    }

    let status_update = match crate::db_slot::is_slot_free(&slot)? {
        false => "REJECTED",
        true => "OCCURRING",
    };

    crate::db_slot::edit_slot_status(slot.id, Some("PENDING"), status_update)?;
    Ok(())
}

#[rocket::head("/admin/event_deny?<slot_id>")]
pub fn event_deny(session: UserSession, slot_id: i64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::edit_slot_status(slot_id, Some("PENDING"), "REJECTED")?;
    Ok(())
}

#[rocket::head("/admin/event_cancel?<slot_id>")]
pub fn event_cancel(session: UserSession, slot_id: i64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    crate::db_slot::edit_slot_status(slot_id, Some("OCCURRING"), "REJECTED")?;
    Ok(())
}

#[rocket::head("/admin/event_suspend?<slot_id>")]
pub fn event_suspend(session: UserSession, slot_id: i64) -> Result<(), Error> {
    if !session.right.admin_event {
        return Err(Error::RightEventMissing);
    };

    let slot: Slot = crate::db_slot::slot_info(slot_id)?;

    if slot.status != "OCCURRING" && slot.status != "REJECTED" {
        return Err(Error::SlotStatusConflict);
    }

    crate::db_slot::edit_slot_status(slot_id, None, "PENDING")?;
    Ok(())
}
