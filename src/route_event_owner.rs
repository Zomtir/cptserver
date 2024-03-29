use crate::common::{Event, EventStatus, User, WebDateTime};
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
    status: Option<EventStatus>,
    location_id: Option<u64>,
) -> Result<Json<Vec<Event>>, Error> {
    let events = crate::db_event::event_list(
        Some(begin.to_naive()),
        Some(end.to_naive()),
        status,
        location_id,
        Some(false),
        None,
        Some(session.user.id),
    )?;
    Ok(Json(events))
}

#[rocket::get("/owner/event_info?<event_id>")]
pub fn event_info(session: UserSession, event_id: u64) -> Result<Json<Event>, Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    Ok(Json(crate::db_event::event_info(event_id)?))
}

// TODO, allow inviting member for draft
// TODO, allow inviting groups for draft
#[rocket::post("/owner/event_edit?<event_id>", format = "application/json", data = "<event>")]
pub fn event_edit(session: UserSession, event_id: u64, mut event: Json<Event>) -> Result<(), Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::common::validate_event_dates(&mut event)?;

    let db_event = crate::db_event::event_info(event_id)?;

    match db_event.status.as_str() {
        "DRAFT" => (),
        _ => return Err(Error::EventStatusConflict),
    }

    crate::db_event::event_edit(event_id, &event)?;
    Ok(())
}

#[rocket::post("/owner/event_password_edit?<event_id>", format = "text/plain", data = "<password>")]
pub fn event_password_edit(session: UserSession, event_id: u64, password: String) -> Result<(), Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db_event::event_password_edit(event_id, password)?;
    Ok(())
}

#[rocket::head("/owner/event_submit?<event_id>")]
pub fn event_submit(session: UserSession, event_id: u64) -> Result<(), Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let event: Event = crate::db_event::event_info(event_id)?;

    // The check is here intentional to be able to return early although it is also checked during is_event_free
    if !crate::common::is_event_valid(&event) {
        return Err(Error::EventWindowInvalid);
    }

    let is_free: bool = crate::db_event::event_free_true(&event)?;

    let status_update = match crate::config::CONFIG_RESERVATION_AUTO_CHECK {
        false => "PENDING",
        true => match is_free {
            true => "OCCURRING",
            false => "REJECTED",
        },
    };

    crate::db_event::event_status_edit(event.id, Some("DRAFT"), status_update)?;
    Ok(())
}

#[rocket::head("/owner/event_withdraw?<event_id>")]
pub fn event_withdraw(session: UserSession, event_id: u64) -> Result<(), Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db_event::event_status_edit(event_id, Some("PENDING"), "DRAFT")?;
    Ok(())
}

#[rocket::head("/owner/event_cancel?<event_id>")]
pub fn event_cancel(session: UserSession, event_id: u64) -> Result<(), Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db_event::event_status_edit(event_id, Some("OCCURRING"), "CANCELED")?;
    Ok(())
}

#[rocket::head("/owner/event_recycle?<event_id>")]
pub fn event_recycle(session: UserSession, event_id: u64) -> Result<(), Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db_event::event_status_edit(event_id, Some("REJECTED"), "DRAFT")?;
    Ok(())
}

#[rocket::head("/owner/event_delete?<event_id>")]
pub fn event_delete(session: UserSession, event_id: u64) -> Result<(), Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let event = crate::db_event::event_info(event_id)?;

    match event.status.as_str() {
        "DRAFT" => (),
        _ => return Err(Error::EventStatusConflict),
    };

    crate::db_event::event_delete(event.id)?;
    Ok(())
}

#[rocket::get("/owner/event_owner_list?<event_id>")]
pub fn event_owner_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let users = crate::db_event::event_owner_list(event_id)?;
    Ok(Json(users))
}

#[rocket::head("/owner/event_owner_add?<event_id>&<user_id>")]
pub fn event_owner_add(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db_event::event_owner_add(event_id, user_id)?;
    Ok(())
}

#[rocket::head("/owner/event_owner_remove?<event_id>&<user_id>")]
pub fn event_owner_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db_event::event_owner_remove(event_id, user_id)?;
    Ok(())
}

#[rocket::get("/owner/event_participant_list?<event_id>")]
pub fn event_participant_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let users = crate::db_event::event_participant_list(event_id)?;
    Ok(Json(users))
}

#[rocket::head("/owner/event_participant_add?<event_id>&<user_id>")]
pub fn event_participant_add(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db_event::event_participant_add(event_id, user_id)?;
    Ok(())
}

#[rocket::head("/owner/event_participant_remove?<event_id>&<user_id>")]
pub fn event_participant_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db_event::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db_event::event_participant_remove(event_id, user_id)?;
    Ok(())
}
