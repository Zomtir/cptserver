use axum::Json;

use crate::common::Affiliation;
use crate::error::Result;
use crate::session::UserSession;

pub fn affiliation_list(
    session: UserSession,
    user_id: Option<u64>,
    organisation_id: Option<u32>,
) -> Result<Json<Vec<Affiliation>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_organisation_read)?;

    let affiliation = crate::db::organisation::affiliation_list(conn, user_id, organisation_id)?;
    Ok(Json(affiliation))
}

pub fn affiliation_info(session: UserSession, user_id: u64, organisation_id: u32) -> Result<Json<Option<Affiliation>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_organisation_read)?;

    let affiliation = crate::db::organisation::affiliation_info(conn, user_id, organisation_id)?;
    Ok(Json(affiliation))
}

pub fn affiliation_create(session: UserSession, user_id: u64, organisation_id: u32) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::affiliation_create(conn, user_id, organisation_id)?;
    Ok(())
}

pub fn affiliation_edit(
    session: UserSession,
    user_id: u64,
    organisation_id: u32,
    affiliation: Json<Affiliation>,
) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::affiliation_edit(conn, user_id, organisation_id, &affiliation)?;
    Ok(())
}

pub fn affiliation_delete(session: UserSession, user_id: u64, organisation_id: u32) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::affiliation_delete(conn, user_id, organisation_id)?;
    Ok(())
}
