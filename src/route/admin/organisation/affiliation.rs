use rocket::serde::json::Json;

use crate::common::Affiliation;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/affiliation_list?<user_id>&<organisation_id>")]
pub fn affiliation_list(
    session: UserSession,
    user_id: Option<u64>,
    organisation_id: Option<u32>,
) -> Result<Json<Vec<Affiliation>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_organisation_read {
        return Err(Error::RightOrganisationMissing);
    };

    let affiliation = crate::db::organisation::affiliation_list(conn, user_id, organisation_id)?;
    Ok(Json(affiliation))
}

#[rocket::get("/admin/affiliation_info?<user_id>&<organisation_id>")]
pub fn affiliation_info(
    session: UserSession,
    user_id: u64,
    organisation_id: u32,
) -> Result<Json<Option<Affiliation>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_organisation_read {
        return Err(Error::RightOrganisationMissing);
    };

    let affiliation = crate::db::organisation::affiliation_info(conn, user_id, organisation_id)?;
    Ok(Json(affiliation))
}

#[rocket::head("/admin/affiliation_create?<user_id>&<organisation_id>")]
pub fn affiliation_create(session: UserSession, user_id: u64, organisation_id: u32) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_organisation_write {
        return Err(Error::RightOrganisationMissing);
    };

    crate::db::organisation::affiliation_create(conn, user_id, organisation_id)?;
    Ok(())
}

#[rocket::post(
    "/admin/affiliation_edit?<user_id>&<organisation_id>",
    format = "application/json",
    data = "<affiliation>"
)]
pub fn affiliation_edit(
    session: UserSession,
    user_id: u64,
    organisation_id: u32,
    affiliation: Json<Affiliation>,
) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_organisation_write {
        return Err(Error::RightOrganisationMissing);
    };

    crate::db::organisation::affiliation_edit(conn, user_id, organisation_id, &affiliation)?;
    Ok(())
}

#[rocket::head("/admin/affiliation_delete?<user_id>&<organisation_id>")]
pub fn affiliation_delete(session: UserSession, user_id: u64, organisation_id: u32) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_organisation_write {
        return Err(Error::RightOrganisationMissing);
    };

    crate::db::organisation::affiliation_delete(conn, user_id, organisation_id)?;
    Ok(())
}
