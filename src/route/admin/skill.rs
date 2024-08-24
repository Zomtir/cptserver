use rocket::serde::json::Json;

use crate::common::Skill;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/skill_list")]
pub fn skill_list(session: UserSession) -> Result<Json<Vec<Skill>>, Error> {
    if !session.right.right_competence_read {
        return Err(Error::RightCompetenceMissing);
    };

    let skills = crate::db::skill::skill_list()?;
    Ok(Json(skills))
}

#[rocket::post("/admin/skill_create", format = "application/json", data = "<skill>")]
pub fn skill_create(session: UserSession, skill: Json<Skill>) -> Result<String, Error> {
    if !session.right.right_competence_write {
        return Err(Error::RightCompetenceMissing);
    };

    let id = crate::db::skill::skill_create(&skill)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/skill_edit?<skill_id>", format = "application/json", data = "<skill>")]
pub fn skill_edit(session: UserSession, skill_id: u32, skill: Json<Skill>) -> Result<(), Error> {
    if !session.right.right_competence_write {
        return Err(Error::RightCompetenceMissing);
    };

    crate::db::skill::skill_edit(skill_id, &skill)?;
    Ok(())
}

#[rocket::head("/admin/skill_delete?<skill_id>")]
pub fn skill_delete(session: UserSession, skill_id: u32) -> Result<(), Error> {
    if !session.right.right_competence_write {
        return Err(Error::RightCompetenceMissing);
    };

    crate::db::skill::skill_delete(skill_id)?;
    Ok(())
}
