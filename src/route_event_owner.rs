use crate::common::{Slot, SlotStatus, User, WebDateTime};
use crate::error::Error;
use crate::session::UserSession;
use rocket::serde::json::Json;

/*
 * ROUTES
 */

#[rocket::get("/owner/event_list?<begin>&<end>&<status>&<location_id>")]
pub fn event_list(
    session: UserSession,
    begin: WebDateTime,
    end: WebDateTime,
    status: Option<SlotStatus>,
    location_id: Option<i64>,
) -> Result<Json<Vec<Slot>>, Error> {
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
        Some(session.user.id),
    )?;
    Ok(Json(slots))
}

#[rocket::get("/owner/event_info?<slot_id>")]
pub fn event_info(session: UserSession, slot_id: i64) -> Result<Json<Slot>, Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    Ok(Json(crate::db_slot::slot_info(slot_id)?))
}

// TODO, allow inviting member for draft
// TODO, allow inviting groups for draft
#[rocket::post("/owner/event_edit?<slot_id>", format = "application/json", data = "<slot>")]
pub fn event_edit(session: UserSession, slot_id: i64, mut slot: Json<Slot>) -> Result<(), Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    crate::common::validate_slot_dates(&mut slot)?;

    let db_slot = crate::db_slot::slot_info(slot_id)?;

    match db_slot.status.as_str() {
        "DRAFT" => (),
        _ => return Err(Error::SlotStatusConflict),
    }

    crate::db_slot::edit_slot(slot_id, &slot)?;
    Ok(())
}

#[rocket::post("/owner/event_edit_password?<slot_id>", format = "text/plain", data = "<password>")]
pub fn event_edit_password(session: UserSession, slot_id: i64, password: String) -> Result<(), Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    crate::db_slot::edit_slot_password(slot_id, password)?;
    Ok(())
}

#[rocket::head("/owner/event_submit?<slot_id>")]
pub fn event_submit(session: UserSession, slot_id: i64) -> Result<(), Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    let slot: Slot = crate::db_slot::slot_info(slot_id)?;

    // The check is here intentional to be able to return early although it is also checked during is_slot_free
    if !crate::common::is_slot_valid(&slot) {
        return Err(Error::SlotWindowInvalid);
    }

    let is_free: bool = crate::db_slot::is_slot_free(&slot)?;

    let status_update = match crate::config::CONFIG_RESERVATION_AUTO_CHECK {
        false => "PENDING",
        true => match is_free {
            true => "OCCURRING",
            false => "REJECTED",
        },
    };

    crate::db_slot::edit_slot_status(slot.id, Some("DRAFT"), status_update)?;
    Ok(())
}

#[rocket::head("/owner/event_withdraw?<slot_id>")]
pub fn event_withdraw(session: UserSession, slot_id: i64) -> Result<(), Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    crate::db_slot::edit_slot_status(slot_id, Some("PENDING"), "DRAFT")?;
    Ok(())
}

#[rocket::head("/owner/event_cancel?<slot_id>")]
pub fn event_cancel(session: UserSession, slot_id: i64) -> Result<(), Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    crate::db_slot::edit_slot_status(slot_id, Some("OCCURRING"), "CANCELED")?;
    Ok(())
}

#[rocket::head("/owner/event_recycle?<slot_id>")]
pub fn event_recycle(session: UserSession, slot_id: i64) -> Result<(), Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    crate::db_slot::edit_slot_status(slot_id, Some("REJECTED"), "DRAFT")?;
    Ok(())
}

#[rocket::head("/owner/event_delete?<slot_id>")]
pub fn event_delete(session: UserSession, slot_id: i64) -> Result<(), Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    let slot = crate::db_slot::slot_info(slot_id)?;

    match slot.status.as_str() {
        "DRAFT" => (),
        _ => return Err(Error::SlotStatusConflict),
    };

    crate::db_slot::slot_delete(slot.id)?;
    Ok(())
}

#[rocket::get("/owner/event_owner_list?<slot_id>")]
pub fn event_owner_list(session: UserSession, slot_id: i64) -> Result<Json<Vec<User>>, Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    let users = crate::db_slot::slot_owner_list(slot_id)?;
    Ok(Json(users))
}

#[rocket::head("/owner/event_owner_add?<slot_id>&<user_id>")]
pub fn event_owner_add(session: UserSession, slot_id: i64, user_id: i64) -> Result<(), Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    crate::db_slot::slot_owner_add(slot_id, user_id)?;
    Ok(())
}

#[rocket::head("/owner/event_owner_remove?<slot_id>&<user_id>")]
pub fn event_owner_remove(session: UserSession, slot_id: i64, user_id: i64) -> Result<(), Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    crate::db_slot::slot_owner_remove(slot_id, user_id)?;
    Ok(())
}

#[rocket::get("/owner/event_participant_list?<slot_id>")]
pub fn event_participant_list(session: UserSession, slot_id: i64) -> Result<Json<Vec<User>>, Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    let users = crate::db_slot::slot_participant_list(slot_id)?;
    Ok(Json(users))
}

#[rocket::head("/owner/event_participant_add?<slot_id>&<user_id>")]
pub fn event_participant_add(session: UserSession, slot_id: i64, user_id: i64) -> Result<(), Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    crate::db_slot::slot_participant_add(slot_id, user_id)?;
    Ok(())
}

#[rocket::head("/owner/event_participant_remove?<slot_id>&<user_id>")]
pub fn event_participant_remove(session: UserSession, slot_id: i64, user_id: i64) -> Result<(), Error> {
    match crate::db_slot::slot_owner_true(slot_id, session.user.id)? {
        false => return Err(Error::SlotOwnerPermission),
        true => (),
    };

    crate::db_slot::slot_participant_remove(slot_id, user_id)?;
    Ok(())
}
