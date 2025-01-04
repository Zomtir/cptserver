use rocket::serde::json::Json;

use crate::common::Organisation;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/organisation_list")]
pub fn organisation_list(session: UserSession) -> Result<Json<Vec<Organisation>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_organisation_read {
        return Err(Error::RightOrganisationMissing);
    };

    let organisations = crate::db::organisation::organisation_list(conn)?;
    Ok(Json(organisations))
}

#[rocket::post("/admin/organisation_create", format = "application/json", data = "<organisation>")]
pub fn organisation_create(session: UserSession, organisation: Json<Organisation>) -> Result<String, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_organisation_write {
        return Err(Error::RightOrganisationMissing);
    };

    let id = crate::db::organisation::organisation_create(conn, &organisation)?;
    Ok(id.to_string())
}

#[rocket::post(
    "/admin/organisation_edit?<organisation_id>",
    format = "application/json",
    data = "<organisation>"
)]
pub fn organisation_edit(
    session: UserSession,
    organisation_id: u32,
    organisation: Json<Organisation>,
) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_organisation_write {
        return Err(Error::RightOrganisationMissing);
    };

    crate::db::organisation::organisation_edit(conn, organisation_id, &organisation)?;
    Ok(())
}

#[rocket::head("/admin/organisation_delete?<organisation_id>")]
pub fn organisation_delete(session: UserSession, organisation_id: u32) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_organisation_write {
        return Err(Error::RightOrganisationMissing);
    };

    crate::db::organisation::organisation_delete(conn, organisation_id)?;
    Ok(())
}
