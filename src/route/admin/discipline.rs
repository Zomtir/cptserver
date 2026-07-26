use axum::Json;

use crate::common::Discipline;
use crate::error::Result;
use crate::session::UserSession;

pub fn discipline_list(session: UserSession) -> Result<Json<Vec<Discipline>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_discipline_read)?;

    let disciplines = crate::db::discipline::discipline_list(conn)?;
    Ok(Json(disciplines))
}

pub fn discipline_create(session: UserSession, discipline: Json<Discipline>) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_discipline_write)?;

    let id = crate::db::discipline::discipline_create(conn, &discipline)?;
    Ok(id.to_string())
}

pub fn discipline_edit(session: UserSession, discipline_id: u32, discipline: Json<Discipline>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_discipline_write)?;

    crate::db::discipline::discipline_edit(conn, discipline_id, &discipline)?;
    Ok(())
}

pub fn discipline_delete(session: UserSession, discipline_id: u32) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_discipline_write)?;

    crate::db::discipline::discipline_delete(conn, discipline_id)?;
    Ok(())
}
