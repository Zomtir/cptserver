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
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_competence_read {
        return Err(Error::RightCompetenceMissing);
    };

    let competences =
        crate::db::competence::competence_list(conn, user_id, skill_id, min.unwrap_or(0), max.unwrap_or(10))?;
    Ok(Json(competences))
}

#[rocket::post("/admin/competence_create", format = "application/json", data = "<competence>")]
pub fn competence_create(session: UserSession, competence: Json<Competence>) -> Result<String, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_competence_write {
        return Err(Error::RightCompetenceMissing);
    };

    let id = crate::db::competence::competence_create(conn, &competence)?;
    Ok(id.to_string())
}

#[rocket::post(
    "/admin/competence_edit?<competence_id>",
    format = "application/json",
    data = "<competence>"
)]
pub fn competence_edit(session: UserSession, competence_id: u64, competence: Json<Competence>) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_competence_write {
        return Err(Error::RightCompetenceMissing);
    };

    crate::db::competence::competence_edit(conn, competence_id, &competence)?;
    Ok(())
}

#[rocket::head("/admin/competence_delete?<competence_id>")]
pub fn competence_delete(session: UserSession, competence_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_competence_write {
        return Err(Error::RightCompetenceMissing);
    };

    crate::db::competence::competence_delete(conn, competence_id)?;
    Ok(())
}
