use axum::extract::State;
use axum::Json;

use crate::common::Team;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub fn sieve_list(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
    role: String,
) -> Result<Json<Vec<(Team, bool)>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_read)?;

    let teams = crate::db::course::attendance::sieve_list(conn, course_id, role)?;
    Ok(Json(teams))
}

pub fn sieve_edit(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
    team_id: u64,
    role: String,
    access: bool,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::attendance::sieve_edit(conn, course_id, team_id, role, access)?;
    Ok(())
}

pub fn sieve_remove(
    State(state): State<AppState>,
    session: UserSession,
    course_id: u32,
    team_id: u64,
    role: String,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_course_write)?;

    crate::db::course::attendance::sieve_remove(conn, course_id, team_id, role)?;
    Ok(())
}
