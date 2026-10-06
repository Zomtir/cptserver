use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::{Affiliation, Club, Event, Term, TermDiscipline, User, WebDate, WebDateTime};
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;

#[derive(Deserialize)]
pub struct TermListQuery {
    pub club_id: Option<u32>,
    pub user_id: Option<u32>,
}

#[derive(Deserialize)]
pub struct ClubStatisticTermsQuery {
    pub point_in_time: WebDate,
}

#[derive(Deserialize)]
pub struct ClubStatisticTeamQuery {
    pub point_in_time: WebDate,
    pub team_id: u32,
}

#[derive(Deserialize)]
pub struct ClubStatisticOrganisationQuery {
    pub point_in_time: WebDate,
    pub organisation_id: u64,
}

#[derive(Deserialize)]
pub struct ClubStatisticAttendanceQuery {
    pub point_in_time_begin: WebDateTime,
    pub point_in_time_end: WebDateTime,
    pub user_id: u64,
    pub role: String,
}

pub async fn club_list(State(state): State<AppState>) -> Result<Json<Vec<Club>>> {
    let conn = &mut state.db.get_conn()?;
    Ok(Json(crate::db::club::club_list(conn)?))
}

pub async fn club_image(State(state): State<AppState>, Path(club_id): Path<u32>) -> Result<Vec<u8>> {
    let conn = &mut state.db.get_conn()?;
    let club = crate::db::club::club_info(conn, club_id)?;
    let path = match club.image_url {
        None => crate::fs::get_resources_path().join("club_image_placeholder.png"),
        Some(url) => crate::fs::get_data_path().join(format!("clubs/{url}")),
    };

    std::fs::read(path).map_err(|_| Error::new(ErrorKind::Filesystem, "Failed to read club image"))
}

pub async fn club_banner(State(state): State<AppState>, Path(club_id): Path<u32>) -> Result<Vec<u8>> {
    let conn = &mut state.db.get_conn()?;
    let club = crate::db::club::club_info(conn, club_id)?;
    let path = match club.banner_url {
        None => crate::fs::get_resources_path().join("club_banner_placeholder.png"),
        Some(url) => crate::fs::get_data_path().join(format!("clubs/{url}")),
    };

    std::fs::read(path).map_err(|_| Error::new(ErrorKind::Filesystem, "Failed to read club banner"))
}

pub async fn club_info(
    State(state): State<AppState>,
    session: UserSession,
    Path(club_id): Path<u32>,
) -> Result<Json<Club>> {
    crate::permission::require_right(session.right.right_club_read)?;
    let conn = &mut state.db.get_conn()?;
    Ok(Json(crate::db::club::club_info(conn, club_id)?))
}

pub async fn club_create(
    State(state): State<AppState>,
    session: UserSession,
    club: Json<Club>,
) -> Result<String> {
    crate::permission::require_right(session.right.right_club_write)?;
    let conn = &mut state.db.get_conn()?;
    Ok(crate::db::club::club_create(conn, &club)?.to_string())
}

pub async fn club_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(club_id): Path<u32>,
    club: Json<Club>,
) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;
    let conn = &mut state.db.get_conn()?;
    crate::db::club::club_edit(conn, club_id, &club)
}

pub async fn club_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(club_id): Path<u32>,
) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;
    let conn = &mut state.db.get_conn()?;
    crate::db::club::club_delete(conn, club_id)
}

pub async fn statistic_terms(
    State(state): State<AppState>,
    session: UserSession,
    Path(club_id): Path<u32>,
    Query(query): Query<ClubStatisticTermsQuery>,
) -> Result<Json<Vec<Term>>> {
    crate::permission::require_right(session.right.right_club_read)?;
    let conn = &mut state.db.get_conn()?;
    Ok(Json(crate::db::club::term_list(
        conn,
        Some(club_id),
        None,
        Some(query.point_in_time.to_naive()),
    )?))
}

pub async fn statistic_members(
    State(state): State<AppState>,
    session: UserSession,
    Path(club_id): Path<u32>,
    Query(query): Query<ClubStatisticTermsQuery>,
) -> Result<Json<Vec<(User, u32)>>> {
    crate::permission::require_right(session.right.right_club_read)?;
    let conn = &mut state.db.get_conn()?;
    Ok(Json(crate::db::club::club_member_leaderboard(
        conn,
        club_id,
        None,
        query.point_in_time.to_naive(),
    )?))
}

pub async fn statistic_team(
    State(state): State<AppState>,
    session: UserSession,
    Path(club_id): Path<u32>,
    Query(query): Query<ClubStatisticTeamQuery>,
) -> Result<Json<Vec<User>>> {
    crate::permission::require_right(session.right.right_club_read)?;
    let conn = &mut state.db.get_conn()?;
    Ok(Json(crate::db::club::club_team_comparison(
        conn,
        club_id,
        query.team_id,
        query.point_in_time.to_naive(),
    )?))
}

pub async fn statistic_organisation(
    State(state): State<AppState>,
    session: UserSession,
    Path(club_id): Path<u32>,
    Query(query): Query<ClubStatisticOrganisationQuery>,
) -> Result<Json<Vec<Affiliation>>> {
    crate::permission::require_right(session.right.right_club_read)?;
    let conn = &mut state.db.get_conn()?;
    Ok(Json(crate::db::club::club_member_organisation(
        conn,
        club_id,
        query.organisation_id,
        None,
        query.point_in_time.to_naive(),
    )?))
}

pub async fn statistic_attendance(
    State(state): State<AppState>,
    session: UserSession,
    Path(club_id): Path<u32>,
    Query(query): Query<ClubStatisticAttendanceQuery>,
) -> Result<Json<Vec<Event>>> {
    crate::permission::require_right(session.right.right_club_read)?;
    let conn = &mut state.db.get_conn()?;
    Ok(Json(crate::db::club::club_user_attendance(
        conn,
        club_id,
        query.user_id,
        query.role.clone(),
        query.point_in_time_begin.to_naive(),
        query.point_in_time_end.to_naive(),
    )?))
}

pub async fn term_list(
    State(state): State<AppState>,
    session: UserSession,
    Query(query): Query<TermListQuery>,
) -> Result<Json<Vec<Term>>> {
    crate::permission::require_right(session.right.right_club_read)?;
    let conn = &mut state.db.get_conn()?;
    Ok(Json(crate::db::club::term_list(conn, query.club_id, query.user_id, None)?))
}

pub async fn term_info(
    State(state): State<AppState>,
    session: UserSession,
    Path(term_id): Path<u64>,
) -> Result<Json<Term>> {
    crate::permission::require_right(session.right.right_club_read)?;
    let conn = &mut state.db.get_conn()?;
    Ok(Json(crate::db::club::term_info(conn, term_id)?))
}

pub async fn term_create(
    State(state): State<AppState>,
    session: UserSession,
    term: Json<Term>,
) -> Result<String> {
    crate::permission::require_right(session.right.right_club_write)?;
    let conn = &mut state.db.get_conn()?;
    Ok(crate::db::club::term_create(conn, &term)?.to_string())
}

pub async fn term_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(term_id): Path<u32>,
    term: Json<Term>,
) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;
    let conn = &mut state.db.get_conn()?;
    crate::db::club::term_edit(conn, term_id, &term)
}

pub async fn term_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(term_id): Path<u32>,
) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;
    let conn = &mut state.db.get_conn()?;
    crate::db::club::term_delete(conn, term_id)
}

pub async fn term_discipline_create(
    State(state): State<AppState>,
    session: UserSession,
    Path(term_id): Path<u32>,
    term_discipline: Json<TermDiscipline>,
) -> Result<String> {
    crate::permission::require_right(session.right.right_club_write)?;
    let conn = &mut state.db.get_conn()?;
    Ok(crate::db::club::term_discipline_create(conn, term_id, &term_discipline)?.to_string())
}

pub async fn term_discipline_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(term_discipline_id): Path<u32>,
    term_discipline: Json<TermDiscipline>,
) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;
    let conn = &mut state.db.get_conn()?;
    crate::db::club::term_discipline_edit(conn, term_discipline_id, &term_discipline)
}

pub async fn term_discipline_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(term_discipline_id): Path<u32>,
) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;
    let conn = &mut state.db.get_conn()?;
    crate::db::club::term_discipline_delete(conn, term_discipline_id)
}
