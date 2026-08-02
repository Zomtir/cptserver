use axum::extract::State;
use axum::Json;

use crate::common::Competence;
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;

pub fn competence_list(
    State(state): State<AppState>,
    session: UserSession,
    user_id: Option<u64>,
    skill_id: Option<u64>,
    min: Option<i16>,
    max: Option<i16>,
) -> Result<Json<Vec<Competence>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_read)?;

    let competences =
        crate::db::competence::competence_list(conn, user_id, skill_id, min.unwrap_or(0), max.unwrap_or(10))?;
    Ok(Json(competences))
}

pub fn competence_info(
    State(state): State<AppState>,
    session: UserSession,
    competence_id: Option<u64>,
) -> Result<Json<Competence>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_read)?;

    let competence = crate::db::competence::competence_info(conn, competence_id)?;

    match competence {
        None => Err(Error::new(ErrorKind::Missing, "Competence not found")),
        Some(c) => Ok(Json(c)),
    }
}

pub fn competence_create(
    State(state): State<AppState>,
    session: UserSession,
    competence: Json<Competence>,
) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    let id = crate::db::competence::competence_create(conn, &competence)?;
    Ok(id.to_string())
}

pub fn competence_edit(
    State(state): State<AppState>,
    session: UserSession,
    competence_id: u64,
    competence: Json<Competence>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    crate::db::competence::competence_edit(conn, competence_id, &competence)?;
    Ok(())
}

pub fn competence_delete(State(state): State<AppState>, session: UserSession, competence_id: u64) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    crate::db::competence::competence_delete(conn, competence_id)?;
    Ok(())
}
