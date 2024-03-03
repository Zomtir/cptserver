use rocket::serde::json::Json;

use crate::common::{Competence, Skill};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/regular/competence_list")]
pub fn competence_list(session: UserSession) -> Result<Json<Vec<Competence>>, Error> {
    let competences = crate::db_competence::competence_list(Some(session.user.id), None, 0, 10)?;
    Ok(Json(competences))
}

#[rocket::get("/regular/competence_summary")]
pub fn competence_summary(session: UserSession) -> Result<Json<Vec<(Skill, i16)>>, Error> {
    let summary = crate::db_competence::competence_summary(session.user.id)?;
    Ok(Json(summary))
}
