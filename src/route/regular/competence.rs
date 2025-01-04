use rocket::serde::json::Json;

use crate::common::{Competence, Skill};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/regular/competence_list")]
pub fn competence_list(session: UserSession) -> Result<Json<Vec<Competence>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let competences = crate::db::competence::competence_list(conn, Some(session.user.id), None, 0, 10)?;
    Ok(Json(competences))
}

#[rocket::get("/regular/competence_summary")]
pub fn competence_summary(session: UserSession) -> Result<Json<Vec<(Skill, i16)>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let summary = crate::db::competence::competence_summary(conn, session.user.id)?;
    Ok(Json(summary))
}
