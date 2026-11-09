use axum::extract::{Path, State};
use axum::Json;

use crate::common::Skill;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub async fn skill_list(State(state): State<AppState>) -> Result<Json<Vec<Skill>>> {
    let conn = &mut state.db.get_conn()?;
    let skills = crate::db::skill::skill_list(conn)?;
    Ok(Json(skills))
}

pub async fn skill_create(State(state): State<AppState>, session: UserSession, skill: Json<Skill>) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    let id = crate::db::skill::skill_create(conn, &skill)?;
    Ok(id.to_string())
}

pub async fn skill_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(skill_id): Path<u32>,
    skill: Json<Skill>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    crate::db::skill::skill_edit(conn, skill_id, &skill)?;
    Ok(())
}

pub async fn skill_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(skill_id): Path<u32>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    crate::db::skill::skill_delete(conn, skill_id)?;
    Ok(())
}
