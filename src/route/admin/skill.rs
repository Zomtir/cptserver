use axum::extract::State;
use axum::Json;

use crate::common::Skill;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub fn skill_list(State(state): State<AppState>, session: UserSession) -> Result<Json<Vec<Skill>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_read)?;

    let skills = crate::db::skill::skill_list(conn)?;
    Ok(Json(skills))
}

pub fn skill_create(State(state): State<AppState>, session: UserSession, skill: Json<Skill>) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    let id = crate::db::skill::skill_create(conn, &skill)?;
    Ok(id.to_string())
}

pub fn skill_edit(
    State(state): State<AppState>,
    session: UserSession,
    skill_id: u32,
    skill: Json<Skill>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    crate::db::skill::skill_edit(conn, skill_id, &skill)?;
    Ok(())
}

pub fn skill_delete(State(state): State<AppState>, session: UserSession, skill_id: u32) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    crate::db::skill::skill_delete(conn, skill_id)?;
    Ok(())
}
