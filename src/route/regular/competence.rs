use axum::Json;

use crate::common::{Competence, Skill};
use crate::error::Result;
use crate::session::UserSession;

pub fn competence_list(session: UserSession) -> Result<Json<Vec<Competence>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let competences = crate::db::competence::competence_list(conn, Some(session.user.id), None, 0, 10)?;
    Ok(Json(competences))
}

pub fn competence_summary(session: UserSession) -> Result<Json<Vec<(Skill, i16)>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let summary = crate::db::competence::competence_summary(conn, session.user.id)?;
    Ok(Json(summary))
}
