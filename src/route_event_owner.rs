use crate::common::{Acceptance, Event, Occurrence, User, WebDateTime};
use crate::error::Error;
use crate::session::UserSession;
use rocket::serde::json::Json;

/*
 * ROUTES
 */

#[rocket::get("/owner/event_list?<begin>&<end>&<location_id>&<occurrence>&<acceptance>")]
pub fn event_list(
    session: UserSession,
    begin: WebDateTime,
    end: WebDateTime,
    location_id: Option<u64>,
    occurrence: Option<Occurrence>,
    acceptance: Option<Acceptance>,
) -> Result<Json<Vec<Event>>, Error> {
    let events = crate::db::event::event_list(
        Some(begin.to_naive()),
        Some(end.to_naive()),
        location_id,
        occurrence,
        acceptance,
        Some(false),
        None,
        Some(session.user.id),
    )?;
    Ok(Json(events))
}

#[rocket::get("/owner/event_info?<event_id>")]
pub fn event_info(session: UserSession, event_id: u64) -> Result<Json<Event>, Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    Ok(Json(crate::db::event::event_info(event_id)?))
}

// TODO, check if new time is free
#[rocket::post("/owner/event_edit?<event_id>", format = "application/json", data = "<event>")]
pub fn event_edit(session: UserSession, event_id: u64, mut event: Json<Event>) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::common::validate_event_dates(&mut event)?;

    crate::db::event::event_edit(event_id, &event)?;
    crate::db::event::event_acceptance_edit(event.id, &Acceptance::Draft)?;
    Ok(())
}

#[rocket::post("/owner/event_password_edit?<event_id>", format = "text/plain", data = "<password>")]
pub fn event_password_edit(session: UserSession, event_id: u64, password: String) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db::event::event_password_edit(event_id, password)?;
    Ok(())
}

#[rocket::head("/owner/event_submit?<event_id>")]
pub fn event_submit(session: UserSession, event_id: u64) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let event: Event = crate::db::event::event_info(event_id)?;

    // The check is here intentional to be able to return early although it is also checked during is_event_free
    if !crate::common::is_event_valid(&event) {
        return Err(Error::EventWindowInvalid);
    }

    let is_free: bool = crate::db::event::event_free_true(&event)?;

    let acceptance = match crate::config::CONFIG_RESERVATION_AUTO_CHECK {
        false => Acceptance::Pending,
        true => match is_free {
            true => Acceptance::Accepted,
            false => Acceptance::Rejected,
        },
    };

    crate::db::event::event_acceptance_edit(event.id, &acceptance)?;
    Ok(())
}

#[rocket::head("/owner/event_withdraw?<event_id>")]
pub fn event_withdraw(session: UserSession, event_id: u64) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db::event::event_acceptance_edit(event_id, &Acceptance::Draft)?;
    Ok(())
}

#[rocket::head("/owner/event_delete?<event_id>")]
pub fn event_delete(session: UserSession, event_id: u64) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db::event::event_delete(event_id)?;
    Ok(())
}

#[rocket::get("/owner/event_owner_list?<event_id>")]
pub fn event_owner_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let users = crate::db::event::owner::event_owner_list(event_id)?;
    Ok(Json(users))
}

#[rocket::head("/owner/event_owner_add?<event_id>&<user_id>")]
pub fn event_owner_add(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db::event::owner::event_owner_add(event_id, user_id)?;
    Ok(())
}

#[rocket::head("/owner/event_owner_remove?<event_id>&<user_id>")]
pub fn event_owner_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db::event::owner::event_owner_remove(event_id, user_id)?;
    Ok(())
}

#[rocket::get("/owner/event_leader_presence_pool?<event_id>")]
pub fn event_leader_presence_pool(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let users = crate::db::event::leader::event_leader_presence_pool(event_id, true)?;
    Ok(Json(users))
}

#[rocket::get("/owner/event_leader_presence_list?<event_id>")]
pub fn event_leader_presence_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let users = crate::db::event::leader::event_leader_presence_list(event_id)?;
    Ok(Json(users))
}

#[rocket::head("/owner/event_leader_presence_add?<event_id>&<user_id>")]
pub fn event_leader_presence_add(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db::event::leader::event_leader_presence_add(event_id, user_id)?;
    Ok(())
}

#[rocket::head("/owner/event_leader_presence_remove?<event_id>&<user_id>")]
pub fn event_leader_presence_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db::event::leader::event_leader_presence_remove(event_id, user_id)?;
    Ok(())
}

#[rocket::get("/owner/event_supporter_presence_pool?<event_id>")]
pub fn event_supporter_presence_pool(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let users = crate::db::event::supporter::event_supporter_presence_pool(event_id, true)?;
    Ok(Json(users))
}

#[rocket::get("/owner/event_supporter_presence_list?<event_id>")]
pub fn event_supporter_presence_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let users = crate::db::event::supporter::event_supporter_presence_list(event_id)?;
    Ok(Json(users))
}

#[rocket::head("/owner/event_supporter_presence_add?<event_id>&<user_id>")]
pub fn event_supporter_presence_add(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db::event::supporter::event_supporter_presence_add(event_id, user_id)?;
    Ok(())
}

#[rocket::head("/owner/event_supporter_presence_remove?<event_id>&<user_id>")]
pub fn event_supporter_presence_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db::event::supporter::event_supporter_presence_remove(event_id, user_id)?;
    Ok(())
}

#[rocket::get("/owner/event_participant_presence_pool?<event_id>")]
pub fn event_participant_presence_pool(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let users = crate::db::event::participant::event_participant_presence_pool(event_id, true)?;
    Ok(Json(users))
}

#[rocket::get("/owner/event_participant_presence_list?<event_id>")]
pub fn event_participant_presence_list(session: UserSession, event_id: u64) -> Result<Json<Vec<User>>, Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let users = crate::db::event::participant::event_participant_presence_list(event_id)?;
    Ok(Json(users))
}

#[rocket::head("/owner/event_participant_presence_add?<event_id>&<user_id>")]
pub fn event_participant_presence_add(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    let pool = crate::db::event::participant::event_participant_presence_pool(event_id, true)?;

    if pool.iter().any(|user| user.id != session.user.id) {
        return Err(Error::RightEventMissing);
    }

    crate::db::event::participant::event_participant_presence_add(event_id, user_id)?;
    Ok(())
}

#[rocket::head("/owner/event_participant_presence_remove?<event_id>&<user_id>")]
pub fn event_participant_presence_remove(session: UserSession, event_id: u64, user_id: u64) -> Result<(), Error> {
    match crate::db::event::owner::event_owner_true(event_id, session.user.id)? {
        false => return Err(Error::EventOwnerPermission),
        true => (),
    };

    crate::db::event::participant::event_participant_presence_remove(event_id, user_id)?;
    Ok(())
}
