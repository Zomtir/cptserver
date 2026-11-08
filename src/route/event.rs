use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::{Acceptance, Affiliation, Credential, Event, Item, Occurrence, User, WebBool, WebDateTime};
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;

pub mod attendance;
pub mod owner;
pub mod service;

#[derive(Deserialize)]
pub struct EventListQuery {
    pub begin: Option<WebDateTime>,
    pub end: Option<WebDateTime>,
    pub location_id: Option<u64>,
    pub occurrence: Option<Occurrence>,
    pub acceptance: Option<Acceptance>,
    pub course_true: Option<WebBool>,
    pub course_id: Option<u32>,
    pub owner_id: Option<u64>,
}

pub async fn event_list(
    State(state): State<AppState>,
    session: UserSession,
    Query(query): Query<EventListQuery>,
) -> Result<Json<Vec<Event>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_read)
        .or(crate::permission::require_course_moderator(conn, query.course_id, Some(session.user.id)))
        .or(crate::permission::is_self(&session, query.owner_id))?;

    let begin = query.begin.map(|date_time| date_time.to_naive());
    let end = query.end.map(|date_time| date_time.to_naive());
    crate::utils::event::verify_event_search_window(begin, end)?;
    let events = crate::db::event::event_list(
        conn,
        begin,
        end,
        query.location_id,
        query.occurrence,
        query.acceptance,
        query.course_true.map(|value| value.to_bool()),
        query.course_id,
        query.owner_id,
    )?;
    Ok(Json(events))
}

pub async fn event_info(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
) -> Result<Json<Event>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_read).or(crate::permission::require_event_owner(
        conn,
        event_id,
        session.user.id,
    ))?;

    Ok(Json(crate::db::event::event_info(conn, event_id)?))
}

#[derive(Deserialize)]
pub struct EventCreateQuery {
    pub course_id: Option<u32>,
}

pub async fn event_create(
    State(state): State<AppState>,
    session: UserSession,
    Query(query): Query<EventCreateQuery>,
    mut event: Json<Event>,
) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    if let Some(course_id) = query.course_id {
        crate::permission::require_right(session.right.right_event_write)
            .or(crate::permission::require_course_moderator(conn, Some(course_id), Some(session.user.id)))?;
        crate::permission::require_right(session.right.right_course_write)
            .or(crate::permission::require_course_moderator(conn, Some(course_id), Some(session.user.id)))?;
    } else {
        crate::permission::require_right(session.right.right_event_write)?;
    }

    crate::utils::event::validate_event_dates(&mut event)?;

    let event_id = crate::db::event::event_create(conn, &event, &Acceptance::Draft, query.course_id)?;
    Ok(event_id.to_string())
}

pub async fn event_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    mut event: Json<Event>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_write)
        .or(crate::permission::require_event_moderator(conn, event_id, session.user.id))
        .or(crate::permission::require_event_owner(conn, event_id, session.user.id))?;

    crate::utils::event::validate_event_dates(&mut event)?;
    crate::db::event::event_edit(conn, event_id, &event)?;
    Ok(())
}

pub async fn event_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_write)
        .or(crate::permission::require_event_moderator(conn, event_id, session.user.id))
        .or(crate::permission::require_event_owner(conn, event_id, session.user.id))?;

    crate::db::event::event_delete(conn, event_id)?;
    Ok(())
}

pub async fn event_acceptance_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    acceptance: String,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_write).or(crate::permission::require_event_owner(
        conn,
        event_id,
        session.user.id,
    ))?;

    let acceptance = Acceptance::try_from(acceptance)?;

    let event = crate::db::event::event_info(conn, event_id)?;
    if !crate::utils::event::is_event_valid(&event) {
        return Err(Error::new(ErrorKind::Invalid, "Event contains invalid data"));
    }

    crate::db::event::event_acceptance_edit(conn, event_id, &acceptance)?;
    Ok(())
}

pub async fn event_submit(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;
    let event = crate::db::event::event_info(conn, event_id)?;
    if !crate::utils::event::is_event_valid(&event) {
        return Err(Error::new(ErrorKind::Invalid, "Event contains invalid data"));
    }

    let acceptance = if !crate::config::EVENT_ACCEPTENCE_AUTO() {
        Acceptance::Pending
    } else if crate::db::event::event_free_true(conn, &event)? {
        Acceptance::Accepted
    } else {
        Acceptance::Rejected
    };
    crate::db::event::event_acceptance_edit(conn, event.id, &acceptance)?;
    Ok(())
}

pub async fn event_withdraw(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_event_owner(conn, event_id, session.user.id)?;
    crate::db::event::event_acceptance_edit(conn, event_id, &Acceptance::Draft)?;
    Ok(())
}

pub async fn event_credential_info(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
) -> Result<Json<Credential>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_read).or(crate::permission::require_event_owner(
        conn,
        event_id,
        session.user.id,
    ))?;

    let (event_key, event_password) = crate::db::login::event_credential(conn, event_id)?;

    Ok(Json(Credential {
        id: None,
        login: Some(event_key),
        password: Some(event_password),
        salt: None,
        since: None,
    }))
}

pub async fn event_password_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    password: String,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_write)
        .or(crate::permission::require_event_moderator(conn, event_id, session.user.id))
        .or(crate::permission::require_event_owner(conn, event_id, session.user.id))?;

    let password = crate::utils::event::validate_clear_password(password)?;
    crate::db::event::event_password_edit(conn, event_id, password)?;
    Ok(())
}

pub async fn event_course_info(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
) -> Result<Json<Option<crate::common::Course>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_read).or(crate::permission::require_event_owner(
        conn,
        event_id,
        session.user.id,
    ))?;

    let course = crate::db::event::event_course_info(conn, event_id)?;
    Ok(Json(course))
}

#[derive(Deserialize)]
pub struct EventCourseEditQuery {
    pub course_id: Option<u32>,
}

pub async fn event_course_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<EventCourseEditQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_write).or(crate::permission::require_event_owner(
        conn,
        event_id,
        session.user.id,
    ))?;

    crate::permission::require_right(session.right.right_course_write).or_else(|_| -> Result<()> {
        if let Some(old_course) = crate::db::event::event_course_info(conn, event_id)? {
            crate::permission::require_course_moderator(conn, Some(old_course.id), Some(session.user.id))?;
        }

        if let Some(new_course) = query.course_id {
            crate::permission::require_course_moderator(conn, Some(new_course), Some(session.user.id))?;
        }

        Ok(())
    })?;

    crate::db::event::event_course_edit(conn, event_id, query.course_id)?;
    Ok(())
}

#[derive(Deserialize)]
pub struct EventStatisticPacklistQuery {
    pub skill_id: u32,
}

pub async fn statistic_packlist(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<EventStatisticPacklistQuery>,
) -> Result<Json<Vec<(User, Item, u32, u32, u32)>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_read)?;

    let stats = crate::db::event::event_statistic_packlist(conn, event_id, query.skill_id)?;
    Ok(Json(stats))
}

#[derive(Deserialize)]
pub struct EventStatisticOrganisationQuery {
    pub organisation_id: u64,
}

pub async fn statistic_organisation(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<EventStatisticOrganisationQuery>,
) -> Result<Json<Vec<Affiliation>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_read)?;

    let stats = crate::db::event::event_statistic_organisation(conn, event_id, query.organisation_id)?;
    Ok(Json(stats))
}
