pub mod attendance;
pub mod owner;

use axum::Json;

use crate::common::{Acceptance, Affiliation, Course, Credential, Event, Item, Occurrence, User, WebBool, WebDateTime};
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;

pub fn event_list(
    session: UserSession,
    begin: Option<WebDateTime>,
    end: Option<WebDateTime>,
    location_id: Option<u64>,
    occurrence: Option<Occurrence>,
    acceptance: Option<Acceptance>,
    course_true: Option<WebBool>,
    course_id: Option<u32>,
    owner_id: Option<u64>,
) -> Result<Json<Vec<Event>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_read)?;

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
        owner_id,
    )?;
    Ok(Json(events))
}

pub fn event_info(session: UserSession, event_id: u64) -> Result<Json<Event>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_read)?;

    Ok(Json(crate::db::event::event_info(conn, event_id)?))
}

pub fn event_credential(session: UserSession, event_id: u64) -> Result<Json<Credential>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_read)?;

    let (event_key, event_pwd) = crate::db::login::event_credential(conn, event_id)?;

    Ok(Json(Credential {
        id: None,
        login: Some(event_key),
        password: Some(event_pwd),
        salt: None,
        since: None,
    }))
}

pub fn event_create(session: UserSession, course_id: Option<u32>, mut event: Json<Event>) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_write)?;

    if course_id.is_some() {
        crate::permission::require_right(session.right.right_course_write)?;
    };

    crate::utils::event::validate_event_dates(&mut event)?;

    let id = crate::db::event::event_create(conn, &event, &Acceptance::Draft, course_id)?;
    Ok(id.to_string())
}

pub fn event_edit(session: UserSession, event_id: u64, mut event: Json<Event>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_write)?;

    crate::utils::event::validate_event_dates(&mut event)?;

    crate::db::event::event_edit(conn, event_id, &event)?;
    Ok(())
}

pub fn event_password_edit(session: UserSession, event_id: u64, password: String) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_write)?;

    let password = crate::utils::event::validate_clear_password(password)?;
    crate::db::event::event_password_edit(conn, event_id, password)?;
    Ok(())
}

pub fn event_course_info(session: UserSession, event_id: u64) -> Result<Json<Option<Course>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_read)?;

    let course = crate::db::event::event_course_info(conn, event_id)?;
    Ok(Json(course))
}

pub fn event_course_edit(session: UserSession, event_id: u64, course_id: Option<u32>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_write)?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::event::event_course_edit(conn, event_id, course_id)?;
    Ok(())
}

pub fn event_delete(session: UserSession, event_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_write)?;

    crate::db::event::event_delete(conn, event_id)?;
    Ok(())
}

pub fn event_accept(session: UserSession, event_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_write)?;

    // Perhaps lock the DB during checking and potentially accepting the request
    let event: Event = crate::db::event::event_info(conn, event_id)?;

    // Check if the event is somewhat reasonable
    if !crate::utils::event::is_event_valid(&event) {
        return Err(Error::new(ErrorKind::Invalid, "Event contains invalid data"));
    }

    crate::db::event::event_acceptance_edit(conn, event.id, &Acceptance::Accepted)?;
    Ok(())
}

pub fn event_reject(session: UserSession, event_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_write)?;

    crate::db::event::event_acceptance_edit(conn, event_id, &Acceptance::Rejected)?;
    Ok(())
}

pub fn event_suspend(session: UserSession, event_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_write)?;

    crate::db::event::event_acceptance_edit(conn, event_id, &Acceptance::Pending)?;
    Ok(())
}

pub fn event_withdraw(session: UserSession, event_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_write)?;

    crate::db::event::event_acceptance_edit(conn, event_id, &Acceptance::Draft)?;
    Ok(())
}

pub fn statistic_packlist(
    session: UserSession,
    event_id: u64,
    skill_id: u32,
) -> Result<Json<Vec<(User, Item, u32, u32, u32)>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_read)?;

    let stats = crate::db::event::event_statistic_packlist(conn, event_id, skill_id)?;
    Ok(Json(stats))
}

pub fn statistic_organisation(
    session: UserSession,
    event_id: u64,
    organisation_id: u64,
) -> Result<Json<Vec<Affiliation>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_event_read)?;

    let stats = crate::db::event::event_statistic_organisation(conn, event_id, organisation_id)?;
    Ok(Json(stats))
}
