use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::Team;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

#[derive(Deserialize)]
pub struct SieveListQuery {
    pub role: String,
}

pub async fn sieve_list(
    State(state): State<AppState>,
    session: UserSession,
    Path(course_id): Path<u32>,
    Query(SieveListQuery { role }): Query<SieveListQuery>,
) -> Result<Json<Vec<(Team, bool)>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let teams = crate::db::course::attendance::sieve_list(conn, course_id, role)?;
    Ok(Json(teams))
}

#[derive(Deserialize)]
pub struct SieveEditQuery {
    pub team_id: u64,
    pub role: String,
    pub access: bool,
}

pub async fn sieve_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(course_id): Path<u32>,
    Query(SieveEditQuery { role, team_id, access }): Query<SieveEditQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::attendance::sieve_edit(conn, course_id, team_id, role, access)?;
    Ok(())
}

#[derive(Deserialize)]
pub struct SieveRemoveQuery {
    pub team_id: u64,
    pub role: String,
}

pub async fn sieve_remove(
    State(state): State<AppState>,
    session: UserSession,
    Path(course_id): Path<u32>,
    Query(SieveRemoveQuery { team_id, role }): Query<SieveRemoveQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::attendance::sieve_remove(conn, course_id, team_id, role)?;
    Ok(())
}
