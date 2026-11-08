use axum::extract::State;
use axum::Json;

use crate::common::{Right, Team, User};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

/* ROUTES */

pub fn team_list(State(state): State<AppState>, session: UserSession) -> Result<Json<Vec<Team>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_team_read)?;

    let teams = crate::db::team::team_list(conn)?;
    Ok(Json(teams))
}

pub fn team_info(State(state): State<AppState>, session: UserSession, team_id: u32) -> Result<Json<Team>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_team_read)?;

    let team = crate::db::team::team_info(conn, &team_id)?;
    Ok(Json(team))
}

pub fn team_create(State(state): State<AppState>, session: UserSession, team: Json<Team>) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_team_write)?;

    let team_id = crate::db::team::team_create(conn, &team)?;
    Ok(team_id.to_string())
}

pub fn team_edit(State(state): State<AppState>, session: UserSession, team_id: u32, team: Json<Team>) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_team_write)?;

    crate::db::team::team_edit(conn, &team_id, &team)?;
    Ok(())
}

pub fn team_right_edit(
    State(state): State<AppState>,
    session: UserSession,
    team_id: u32,
    right: Json<Right>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_team_write)?;

    crate::db::team::team_right_edit(conn, &team_id, &right)?;
    Ok(())
}

pub fn team_delete(State(state): State<AppState>, session: UserSession, team_id: u32) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_team_write)?;

    crate::db::team::team_delete(conn, &team_id)?;
    Ok(())
}

pub fn team_member_list(State(state): State<AppState>, session: UserSession, team_id: u32) -> Result<Json<Vec<User>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_team_read)?;

    let users = crate::db::team::team_member_list(conn, team_id)?;
    Ok(Json(users))
}

pub fn team_member_add(State(state): State<AppState>, session: UserSession, team_id: u32, user_id: u32) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_team_write)?;

    crate::db::team::team_member_add(conn, &team_id, &user_id)?;
    Ok(())

    // TODO: remove/add permissions of currently logged-in users
}

pub fn team_member_remove(
    State(state): State<AppState>,
    session: UserSession,
    team_id: u32,
    user_id: u32,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_team_write)?;

    crate::db::team::team_member_remove(conn, &team_id, &user_id)?;
    Ok(())

    // TODO: remove/add permissions of currently logged-in users
}
