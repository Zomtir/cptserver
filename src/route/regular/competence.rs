use axum::extract::State;
use axum::Json;

use crate::common::{Competence, Skill};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub fn competence_list(State(state): State<AppState>, session: UserSession) -> Result<Json<Vec<Competence>>> {
    let conn = &mut state.db.get_conn()?;
    let competences = crate::db::competence::competence_list(conn, Some(session.user.id), None, 0, 10)?;
    Ok(Json(competences))
}

pub fn competence_summary(State(state): State<AppState>, session: UserSession) -> Result<Json<Vec<(Skill, i16)>>> {
    let conn = &mut state.db.get_conn()?;
    let summary = crate::db::competence::competence_summary(conn, session.user.id)?;
    Ok(Json(summary))
}
