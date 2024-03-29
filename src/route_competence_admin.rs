use rocket::serde::json::Json;

use crate::common::Competence;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/competence_list?<user_id>&<skill_id>&<min>&<max>")]
pub fn competence_list(
    session: UserSession,
    user_id: Option<u64>,
    skill_id: Option<u64>,
    min: Option<i16>,
    max: Option<i16>,
) -> Result<Json<Vec<Competence>>, Error> {
    if !session.right.admin_competence {
        return Err(Error::RightCompetenceMissing);
    };

    let competences = crate::db_competence::competence_list(user_id, skill_id, min.unwrap_or(0), max.unwrap_or(10))?;
    Ok(Json(competences))
}

#[rocket::post("/admin/competence_create", format = "application/json", data = "<competence>")]
pub fn competence_create(session: UserSession, competence: Json<Competence>) -> Result<String, Error> {
    if !session.right.admin_competence {
        return Err(Error::RightCompetenceMissing);
    };

    let id = crate::db_competence::competence_create(&competence)?;
    Ok(id.to_string())
}

#[rocket::post(
    "/admin/competence_edit?<competence_id>",
    format = "application/json",
    data = "<competence>"
)]
pub fn competence_edit(session: UserSession, competence_id: u64, competence: Json<Competence>) -> Result<(), Error> {
    if !session.right.admin_competence {
        return Err(Error::RightCompetenceMissing);
    };

    crate::db_competence::competence_edit(competence_id, &competence)?;
    Ok(())
}

#[rocket::head("/admin/competence_delete?<competence_id>")]
pub fn competence_delete(session: UserSession, competence_id: u64) -> Result<(), Error> {
    if !session.right.admin_competence {
        return Err(Error::RightCompetenceMissing);
    };

    crate::db_competence::competence_delete(competence_id)?;
    Ok(())
}
