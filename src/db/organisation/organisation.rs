use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::Organisation;
use crate::error::ErrorKind;

pub fn organisation_list(conn: &mut PooledConn) -> Result<Vec<Organisation>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT organisation_id, abbreviation, name
        FROM organisations;",
    )?;

    let params = params::Params::Empty;

    let map = Organisation::sql_map();

    let orgs = conn.exec_map(&stmt, &params, &map)?;
    Ok(orgs)
}

pub fn organisation_info(conn: &mut PooledConn, organisation_id: u32) -> Result<Organisation, ErrorKind> {
    let stmt = conn.prep(
        "SELECT organisation_id, abbreviation, name
        FROM organisations
        WHERE organisation_id = :organisation_id;",
    )?;

    let params = params! {
        "organisation_id" => &organisation_id,
    };

    let map = Organisation::sql_map();

    let mut orgs = conn.exec_map(&stmt, &params, &map)?;
    if orgs.is_empty() {
        return Err(ErrorKind::OrganisationMissing);
    }
    Ok(orgs.remove(0))
}

pub fn organisation_create(conn: &mut PooledConn, organisation: &Organisation) -> Result<u32, ErrorKind> {
    let stmt = conn.prep(
        "INSERT INTO organisations (abbreviation, name)
        VALUES (:abbreviation, :name)",
    )?;

    let params = params! {
        "abbreviation" => &organisation.abbreviation,
        "name" => &organisation.name,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn organisation_edit(
    conn: &mut PooledConn,
    organisation_id: u32,
    organisation: &Organisation,
) -> Result<(), ErrorKind> {
    let stmt = conn.prep(
        "UPDATE organisations SET
            abbreviation = :abbreviation,
            name = :name,
        WHERE organisation_id = :organisation_id",
    )?;

    let params = params! {
        "organisation_id" => &organisation_id,
        "abbreviation" => &organisation.abbreviation,
        "name" => &organisation.name,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn organisation_delete(conn: &mut PooledConn, organisation_id: u32) -> Result<(), ErrorKind> {
    let stmt = conn.prep("DELETE s FROM organisations s WHERE s.organisation_id = :organisation_id")?;

    let params = params! {
        "organisation_id" => organisation_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
