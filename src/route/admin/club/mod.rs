pub mod term;
pub mod term_discipline;

use axum::extract::State;
use axum::Json;

use crate::common::{Affiliation, Club, Event, Term, User, WebDate, WebDateTime};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub fn club_list(State(state): State<AppState>,session: UserSession) -> Result<Json<Vec<Club>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_club_read)?;

    let clubs = crate::db::club::club_list(conn)?;
    Ok(Json(clubs))
}

pub fn club_info(State(state): State<AppState>, session: UserSession, club_id: u32) -> Result<Json<Club>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_club_read)?;

    let club = crate::db::club::club_info(conn, club_id)?;

    Ok(Json(club))
}

pub fn club_create(State(state): State<AppState>, session: UserSession, club: Json<Club>) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_club_write)?;

    let id = crate::db::club::club_create(conn, &club)?;
    Ok(id.to_string())
}

pub fn club_edit(State(state): State<AppState>, session: UserSession, club_id: u32, club: Json<Club>) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_club_write)?;

    crate::db::club::club_edit(conn, club_id, &club)?;
    Ok(())
}

pub fn club_delete(State(state): State<AppState>, session: UserSession, club_id: u32) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_club_write)?;

    crate::db::club::club_delete(conn, club_id)?;
    Ok(())
}

/* STATISTICS */

pub fn statistic_terms(
    State(state): State<AppState>,
    session: UserSession,
    club_id: u32,
    point_in_time: WebDate,
) -> Result<Json<Vec<Term>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_club_read)?;

    let terms = crate::db::club::term_list(conn, Some(club_id), None, Some(point_in_time.to_naive()))?;
    Ok(Json(terms))
}

pub fn statistic_members(
    State(state): State<AppState>,
    session: UserSession,
    club_id: u32,
    point_in_time: WebDate,
) -> Result<Json<Vec<(User, u32)>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_club_read)?;

    let leaderboard = crate::db::club::club_member_leaderboard(conn, club_id, None, point_in_time.to_naive())?;
    Ok(Json(leaderboard))
}

pub fn statistic_team(
    State(state): State<AppState>,
    session: UserSession,
    club_id: u32,
    point_in_time: WebDate,
    team_id: u32,
) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_club_read)?;

    let list = crate::db::club::club_team_comparison(conn, club_id, team_id, point_in_time.to_naive())?;
    Ok(Json(list))
}

pub fn statistic_organisation(
    State(state): State<AppState>,
    session: UserSession,
    club_id: u32,
    organisation_id: u64,
    point_in_time: WebDate,
) -> Result<Json<Vec<Affiliation>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_club_read)?;

    let list =
        crate::db::club::club_member_organisation(conn, club_id, organisation_id, None, point_in_time.to_naive())?;
    Ok(Json(list))
}

pub fn statistic_attendance(
    State(state): State<AppState>,
    session: UserSession,
    club_id: u32,
    user_id: u64,
    role: String,
    time_window_begin: WebDateTime,
    time_window_end: WebDateTime,
) -> Result<Json<Vec<Event>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_club_read)?;

    let stats = crate::db::club::club_user_attendance(
        conn,
        club_id,
        user_id,
        role,
        time_window_begin.to_naive(),
        time_window_end.to_naive(),
    )?;
    Ok(Json(stats))
}
