use crate::common::{Acceptance, Confirmation, Event, Occurrence, WebBool, WebDateTime};
use crate::error::Error;
use crate::session::UserSession;
use rocket::serde::json::Json;

/*
 * ROUTES
 */

#[rocket::get("/regular/event_list?<begin>&<end>&<location_id>&<occurrence>&<acceptance>&<course_true>&<course_id>")]
pub fn event_list(
    _session: UserSession,
    begin: Option<WebDateTime>,
    end: Option<WebDateTime>,
    location_id: Option<u64>,
    occurrence: Option<Occurrence>,
    acceptance: Option<Acceptance>,
    course_true: Option<WebBool>,
    course_id: Option<u32>,
) -> Result<Json<Vec<Event>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;

    let begin = begin.map(|dt| dt.to_naive());
    let end = end.map(|dt| dt.to_naive());
    crate::utils::event::verify_event_search_window(begin, end)?;

    let events = crate::db::event::event_list(
        conn,
        begin,
        end,
        location_id,
        occurrence,
        acceptance,
        course_true.map(|b| b.to_bool()),
        course_id,
        None,
    )?;
    Ok(Json(events))
}

#[rocket::post("/regular/event_create", format = "application/json", data = "<event>")]
pub fn event_create(session: UserSession, mut event: Json<Event>) -> Result<String, Error> {
    crate::utils::event::validate_event_dates(&mut event)?;
    let conn = &mut crate::utils::db::get_db_conn()?;

    let event_id = crate::db::event::event_create(conn, &event, &Acceptance::Draft, None)?;
    crate::db::event::owner::event_owner_add(conn, event_id, session.user.id)?;
    Ok(event_id.to_string())
}

#[rocket::get("/regular/event_owner_true?<event_id>")]
pub fn event_owner_true(session: UserSession, event_id: u64) -> Result<Json<bool>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let condition = crate::db::event::owner::event_owner_true(conn, event_id, session.user.id)?;
    Ok(Json(condition))
}

#[rocket::get("/regular/event_attendance_presence_true?<event_id>&<role>")]
pub fn event_attendance_presence_true(session: UserSession, event_id: u64, role: String) -> Result<Json<bool>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let condition =
        crate::db::event::attendance::event_attendance_presence_true(conn, event_id, session.user.id, &role)?;
    Ok(Json(condition))
}

#[rocket::head("/regular/event_attendance_presence_add?<event_id>&<role>")]
pub fn event_attendance_presence_add(session: UserSession, event_id: u64, role: String) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let pool = crate::db::event::attendance::event_attendance_presence_pool(conn, event_id, &role, true)?;

    if !pool.iter().any(|user| user.id == session.user.id) {
        return Err(Error::EventPresenceForbidden);
    }

    crate::db::event::attendance::event_attendance_presence_add(conn, event_id, session.user.id, &role)?;
    Ok(())
}

#[rocket::head("/regular/event_attendance_presence_remove?<event_id>&<role>")]
pub fn event_attendance_presence_remove(session: UserSession, event_id: u64, role: String) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::db::event::attendance::event_attendance_presence_remove(conn, event_id, session.user.id, &role)?;
    Ok(())
}

#[rocket::get("/regular/event_bookmark_true?<event_id>")]
pub fn event_bookmark_true(session: UserSession, event_id: u64) -> Result<Json<bool>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    // TODO check if you can participate

    let bookmark = crate::db::event::event_bookmark_true(conn, event_id, session.user.id)?;
    Ok(Json(bookmark))
}

#[rocket::head("/regular/event_bookmark_edit?<event_id>&<bookmark>")]
pub fn event_bookmark_edit(session: UserSession, event_id: u64, bookmark: bool) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    // TODO check if you can participate

    match bookmark {
        true => crate::db::event::event_bookmark_add(conn, event_id, session.user.id)?,
        false => crate::db::event::event_bookmark_remove(conn, event_id, session.user.id)?,
    }
    Ok(())
}

#[rocket::get("/regular/event_attendance_registration_info?<event_id>&<role>")]
pub fn event_attendance_registration_info(session: UserSession, event_id: u64, role: String) -> Result<String, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    // TODO check if you can register (requirement)

    let status =
        crate::db::event::attendance::event_attendance_registration_info(conn, event_id, session.user.id, role)?;
    Ok(status.to_string())
}

#[rocket::head("/regular/event_attendance_registration_edit?<event_id>&<role>&<status>")]
pub fn event_attendance_registration_edit(
    session: UserSession,
    event_id: u64,
    role: String,
    status: Confirmation,
) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    // TODO check if you can register (requirement)

    match status {
        Confirmation::Null => {
            crate::db::event::attendance::event_attendance_registration_remove(conn, event_id, session.user.id, role)?
        }
        _ => crate::db::event::attendance::event_attendance_registration_edit(
            conn,
            event_id,
            session.user.id,
            role,
            status,
        )?,
    }
    Ok(())
}
