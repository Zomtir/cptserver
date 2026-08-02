pub mod attendance;
pub mod moderator;

use axum::extract::State;
use axum::Json;

use crate::common::{Acceptance, Course, Event, Requirement, User, WebBool};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub fn course_list(
    State(state): State<AppState>,
    session: UserSession,
    mod_id: Option<u64>,
    active: Option<WebBool>,
    public: Option<WebBool>,
) -> Result<Json<Vec<Course>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let courses =
        crate::db::course::course_list(conn, mod_id, active.map(|b| b.to_bool()), public.map(|b| b.to_bool()))?;
    Ok(Json(courses))
}

pub fn course_create(State(state): State<AppState>, session: UserSession, course: Json<Course>) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    let id = crate::db::course::course_create(conn, &course)?;
    Ok(id.to_string())
}

pub fn course_edit(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
    course: Json<Course>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::course_edit(conn, course_id, &course)?;
    Ok(())
}

pub fn course_delete(State(state): State<AppState>, session: UserSession, course_id: u32) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::course_delete(conn, course_id)?;
    Ok(())
}

pub fn course_event_list(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
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

pub fn course_requirement_list(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
) -> Result<Json<Vec<Requirement>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let reqs = crate::db::course::course_requirement_list(conn, course_id)?;
    Ok(Json(reqs))
}

pub fn course_requirement_add(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
    skill_id: u32,
    rank: u32,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::course_requirement_add(conn, course_id, skill_id, rank)?;
    Ok(())
}

pub fn course_requirement_remove(
    State(state): State<AppState>,
    session: UserSession,
    requirement_id: u64,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::course_requirement_remove(conn, requirement_id)?;
    Ok(())
}

pub fn course_club_info(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u64,
) -> Result<Json<Option<u32>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let club_id = crate::db::course::course_club_info(conn, course_id)?;
    Ok(Json(club_id))
}

pub fn course_club_edit(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u64,
    club_id: Option<u32>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;
    crate::permission::require_right(session.right.right_club_write)?;

    crate::db::course::course_club_edit(conn, course_id, club_id)?;
    Ok(())
}

pub fn course_statistic_class(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
) -> Result<Json<Vec<(Event, u64, u64, u64, u64)>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let stats = crate::db::course::course_statistic_class(conn, course_id)?;
    Ok(Json(stats))
}

pub fn course_statistic_attendance(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
    role: String,
) -> Result<Json<Vec<(User, u64)>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let stats = crate::db::course::course_statistic_attendance(conn, course_id, role)?;
    Ok(Json(stats))
}

pub fn course_statistic_attendance1(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
    user_id: u64,
    role: String,
) -> Result<Json<Vec<Event>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let stats = crate::db::course::course_statistic_attendance1(conn, course_id, user_id, role)?;
    Ok(Json(stats))
}
