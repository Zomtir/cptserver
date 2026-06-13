use rocket::serde::json::Json;

use crate::common::Discipline;
use crate::error::Result;
use crate::session::UserSession;

#[rocket::get("/admin/discipline_list")]
pub fn discipline_list(session: UserSession) -> Result<Json<Vec<Discipline>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_discipline_read)?;

    let disciplines = crate::db::discipline::discipline_list(conn)?;
    Ok(Json(disciplines))
}

#[rocket::post("/admin/discipline_create", format = "application/json", data = "<discipline>")]
pub fn discipline_create(session: UserSession, discipline: Json<Discipline>) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_discipline_write)?;

    let id = crate::db::discipline::discipline_create(conn, &discipline)?;
    Ok(id.to_string())
}

#[rocket::post(
    "/admin/discipline_edit?<discipline_id>",
    format = "application/json",
    data = "<discipline>"
)]
pub fn discipline_edit(session: UserSession, discipline_id: u32, discipline: Json<Discipline>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_discipline_write)?;

    crate::db::discipline::discipline_edit(conn, discipline_id, &discipline)?;
    Ok(())
}

#[rocket::head("/admin/discipline_delete?<discipline_id>")]
pub fn discipline_delete(session: UserSession, discipline_id: u32) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_discipline_write)?;

    crate::db::discipline::discipline_delete(conn, discipline_id)?;
    Ok(())
}
