use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::{Acceptance, Course, Event, WebBool};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub mod attendance;
pub mod moderator;
pub mod requirement;
pub mod statistic;

#[derive(Deserialize)]
pub struct CourseAllQuery {
    pub active: Option<WebBool>,
    pub public: Option<WebBool>,
    pub mod_id: Option<u64>,
}

pub async fn course_all(
    State(state): State<AppState>,
    session: UserSession,
    Query(query): Query<CourseAllQuery>,
) -> Result<Json<Vec<Course>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let courses = crate::db::course::course_list(
        conn,
        query.mod_id,
        query.active.map(|b| b.to_bool()),
        query.public.map(|b| b.to_bool()),
    )?;
    Ok(Json(courses))
}

pub async fn course_list(State(state): State<AppState>) -> Result<Json<Vec<Course>>> {
    let conn = &mut state.db.get_conn()?;

    let courses = crate::db::course::course_list(conn, None, Some(true), Some(true))?;
    Ok(Json(courses))
}

pub async fn course_availability(
    State(state): State<AppState>,
    session: UserSession,
    Path(user_id): Path<u64>,
) -> Result<Json<Vec<crate::common::Course>>> {
    crate::permission::require_right(session.right.right_event_read)
        .or(crate::permission::is_self(&session, Some(user_id)))?;

    let conn = &mut state.db.get_conn()?;
    let courses = crate::db::course::course_available(conn, user_id)?;
    Ok(Json(courses))
}

#[derive(Deserialize)]
pub struct CourseResponsibilityQuery {
    pub active: Option<WebBool>,
    pub public: Option<WebBool>,
}

pub async fn course_responsibility(
    State(state): State<AppState>,
    session: UserSession,
    Path(user_id): Path<u64>,
    Query(query): Query<CourseResponsibilityQuery>,
) -> Result<Json<Vec<Course>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)
        .or(crate::permission::is_self(&session, Some(user_id)))?;

    let courses = crate::db::course::course_list(
        conn,
        Some(user_id),
        query.active.map(|b| b.to_bool()),
        query.public.map(|b| b.to_bool()),
    )?;
    Ok(Json(courses))
}

pub async fn course_create(
    State(state): State<AppState>,
    session: UserSession,
    course: Json<Course>,
) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    let id = crate::db::course::course_create(conn, &course)?;
    Ok(id.to_string())
}

pub async fn course_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(course_id): Path<u32>,
    course: Json<Course>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::course_edit(conn, course_id, &course)?;
    Ok(())
}

pub async fn course_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(course_id): Path<u32>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::course_delete(conn, course_id)?;
    Ok(())
}

pub async fn course_event_list(
    State(state): State<AppState>,
    session: UserSession,
    Path(course_id): Path<u32>,
) -> Result<Json<Vec<Event>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let events = crate::db::event::event_list(
        conn,
        None,
        None,
        None,
        None,
        Some(Acceptance::Accepted),
        Some(true),
        Some(course_id),
        None,
    )?;
    Ok(Json(events))
}

pub async fn course_club_info(
    State(state): State<AppState>,
    session: UserSession,
    Path(course_id): Path<u64>,
) -> Result<Json<Option<u32>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let club_id = crate::db::course::course_club_info(conn, course_id)?;
    Ok(Json(club_id))
}

pub async fn course_club_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(course_id): Path<u64>,
    Query(club_id): Query<Option<u32>>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;
    crate::permission::require_right(session.right.right_club_write)?;

    crate::db::course::course_club_edit(conn, course_id, club_id)?;
    Ok(())
}
