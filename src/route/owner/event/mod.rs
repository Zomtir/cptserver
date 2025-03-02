pub mod attendance;
pub mod owner;

use crate::common::{Acceptance, Event, Occurrence, WebDateTime};
use crate::error::Error;
use crate::session::UserSession;
use rocket::serde::json::Json;

#[rocket::get("/owner/event_list?<begin>&<end>&<location_id>&<occurrence>&<acceptance>")]
pub fn event_list(
    session: UserSession,
    begin: WebDateTime,
    end: WebDateTime,
    location_id: Option<u64>,
    occurrence: Option<Occurrence>,
    acceptance: Option<Acceptance>,
) -> Result<Json<Vec<Event>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;

    let begin = Some(begin.to_naive());
    let end = Some(end.to_naive());
    crate::utils::event::verify_event_search_window(begin, end)?;

    let events = crate::db::event::event_list(
        conn,
        begin,
        end,
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
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !crate::db::event::owner::event_owner_true(conn, event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    Ok(Json(crate::db::event::event_info(conn, event_id)?))
}

// TODO, check if new time is free
#[rocket::post("/owner/event_edit?<event_id>", format = "application/json", data = "<event>")]
pub fn event_edit(session: UserSession, event_id: u64, mut event: Json<Event>) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !crate::db::event::owner::event_owner_true(conn, event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    crate::utils::event::validate_event_dates(&mut event)?;

    crate::db::event::event_edit(conn, event_id, &event)?;
    crate::db::event::event_acceptance_edit(conn, event.id, &Acceptance::Draft)?;
    Ok(())
}

#[rocket::post("/owner/event_password_edit?<event_id>", format = "text/plain", data = "<password>")]
pub fn event_password_edit(session: UserSession, event_id: u64, password: String) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !crate::db::event::owner::event_owner_true(conn, event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    let password = crate::utils::event::validate_clear_password(password)?;
    crate::db::event::event_password_edit(conn, event_id, password)?;
    Ok(())
}

#[rocket::get("/owner/event_course_info?<event_id>")]
pub fn event_course_info(session: UserSession, event_id: u64) -> Result<Json<Option<u32>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !crate::db::event::owner::event_owner_true(conn, event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    let course_id = crate::db::event::event_course_info(conn, event_id)?;
    Ok(Json(course_id))
}

#[rocket::head("/owner/event_course_edit?<event_id>&<course_id>")]
pub fn event_course_edit(session: UserSession, event_id: u64, course_id: Option<u32>) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !crate::db::event::owner::event_owner_true(conn, event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    if let Some(old_id) = crate::db::event::event_course_info(conn, event_id)? {
        if !crate::db::course::moderator::course_moderator_true(conn, old_id, session.user.id)? {
            return Err(Error::CourseModeratorPermission);
        };
    };

    if let Some(new_id) = course_id {
        if !crate::db::course::moderator::course_moderator_true(conn, new_id, session.user.id)? {
            return Err(Error::CourseModeratorPermission);
        };
    };

    crate::db::event::event_course_edit(conn, event_id, course_id)?;
    Ok(())
}

#[rocket::head("/owner/event_submit?<event_id>")]
pub fn event_submit(session: UserSession, event_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !crate::db::event::owner::event_owner_true(conn, event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    let event: Event = crate::db::event::event_info(conn, event_id)?;

    // The check is here intentional to be able to return early although it is also checked during is_event_free
    if !crate::utils::event::is_event_valid(&event) {
        return Err(Error::EventWindowInvalid);
    }

    let is_free: bool = crate::db::event::event_free_true(conn, &event)?;

    let acceptance = match crate::config::EVENT_ACCEPTENCE_AUTO() {
        false => Acceptance::Pending,
        true => match is_free {
            true => Acceptance::Accepted,
            false => Acceptance::Rejected,
        },
    };

    crate::db::event::event_acceptance_edit(conn, event.id, &acceptance)?;
    Ok(())
}

#[rocket::head("/owner/event_withdraw?<event_id>")]
pub fn event_withdraw(session: UserSession, event_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !crate::db::event::owner::event_owner_true(conn, event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    crate::db::event::event_acceptance_edit(conn, event_id, &Acceptance::Draft)?;
    Ok(())
}

#[rocket::head("/owner/event_delete?<event_id>")]
pub fn event_delete(session: UserSession, event_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !crate::db::event::owner::event_owner_true(conn, event_id, session.user.id)? {
        return Err(Error::EventOwnerPermission);
    };

    crate::db::event::event_delete(conn, event_id)?;
    Ok(())
}
