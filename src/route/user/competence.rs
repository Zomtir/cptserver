use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::{Competence, Skill};
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;

#[derive(Deserialize)]
pub struct CompetenceListQuery {
    pub skill_id: Option<u64>,
    pub min: Option<i16>,
    pub max: Option<i16>,
}

pub async fn competence_list(
    State(state): State<AppState>,
    session: UserSession,
    Path(user_id): Path<Option<u64>>,
    Query(query): Query<CompetenceListQuery>,
) -> Result<Json<Vec<Competence>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_read)
        .or(crate::permission::is_self(&session, user_id))?;

    let competences = crate::db::competence::competence_list(
        conn,
        user_id,
        query.skill_id,
        query.min.unwrap_or(0),
        query.max.unwrap_or(10),
    )?;
    Ok(Json(competences))
}

pub async fn competence_info(
    State(state): State<AppState>,
    session: UserSession,
    Path((user_id, competence_id)): Path<(Option<u64>, Option<u64>)>,
) -> Result<Json<Competence>> {
    let conn = &mut state.db.get_conn()?;

    crate::permission::require_right(session.right.right_competence_read)
        .or(crate::permission::is_self(&session, user_id))?;

    let competence = crate::db::competence::competence_info(conn, competence_id)?;

    match competence {
        None => Err(Error::new(ErrorKind::Missing, "Competence not found")),
        Some(c) => Ok(Json(c)),
    }
}

pub async fn competence_summary(
    State(state): State<AppState>,
    session: UserSession,
    Path(user_id): Path<Option<u64>>,
) -> Result<Json<Vec<(Skill, i16)>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_read)
        .or(crate::permission::is_self(&session, user_id))?;

    let summary = crate::db::competence::competence_summary(conn, session.user.id)?;
    Ok(Json(summary))
}

pub async fn competence_create(
    State(state): State<AppState>,
    session: UserSession,
    competence: Json<Competence>,
) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    let id = crate::db::competence::competence_create(conn, &competence)?;
    Ok(id.to_string())
}

pub async fn competence_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(competence_id): Path<u64>,
    competence: Json<Competence>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    crate::db::competence::competence_edit(conn, competence_id, &competence)?;
    Ok(())
}

pub async fn competence_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(competence_id): Path<u64>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    crate::db::competence::competence_delete(conn, competence_id)?;
    Ok(())
}
