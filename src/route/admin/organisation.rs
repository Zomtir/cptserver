use rocket::serde::json::Json;

use crate::common::Organisation;
use crate::error::Result;
use crate::session::UserSession;

mod affiliation;
pub use affiliation::*;

#[rocket::get("/admin/organisation_list")]
pub fn organisation_list(session: UserSession) -> Result<Json<Vec<Organisation>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_organisation_read)?;

    let organisations = crate::db::organisation::organisation_list(conn)?;
    Ok(Json(organisations))
}

#[rocket::get("/admin/organisation_info?<organisation_id>")]
pub fn organisation_info(session: UserSession, organisation_id: u32) -> Result<Json<Organisation>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_organisation_read)?;

    let organisation = crate::db::organisation::organisation_info(conn, organisation_id)?;
    Ok(Json(organisation))
}

#[rocket::post("/admin/organisation_create", format = "application/json", data = "<organisation>")]
pub fn organisation_create(session: UserSession, organisation: Json<Organisation>) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    let id = crate::db::organisation::organisation_create(conn, &organisation)?;
    Ok(id.to_string())
}

#[rocket::post(
    "/admin/organisation_edit?<organisation_id>",
    format = "application/json",
    data = "<organisation>"
)]
pub fn organisation_edit(session: UserSession, organisation_id: u32, organisation: Json<Organisation>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::organisation_edit(conn, organisation_id, &organisation)?;
    Ok(())
}

#[rocket::head("/admin/organisation_delete?<organisation_id>")]
pub fn organisation_delete(session: UserSession, organisation_id: u32) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::organisation_delete(conn, organisation_id)?;
    Ok(())
}
