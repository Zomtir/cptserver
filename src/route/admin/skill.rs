use axum::Json;

use crate::common::Skill;
use crate::error::Result;
use crate::session::UserSession;

pub fn skill_list(session: UserSession) -> Result<Json<Vec<Skill>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_competence_read)?;

    let skills = crate::db::skill::skill_list(conn)?;
    Ok(Json(skills))
}

pub fn skill_create(session: UserSession, skill: Json<Skill>) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    let id = crate::db::skill::skill_create(conn, &skill)?;
    Ok(id.to_string())
}

pub fn skill_edit(session: UserSession, skill_id: u32, skill: Json<Skill>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    crate::db::skill::skill_edit(conn, skill_id, &skill)?;
    Ok(())
}

pub fn skill_delete(session: UserSession, skill_id: u32) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_competence_write)?;

    crate::db::skill::skill_delete(conn, skill_id)?;
    Ok(())
}
