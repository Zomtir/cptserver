use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::Organisation;
use crate::error::Error;

pub fn organisation_list(conn: &mut PooledConn) -> Result<Vec<Organisation>, Error> {
    let stmt = conn.prep(
        "SELECT organisation_id, abbreviation, name
        FROM organisations;",
    )?;

    let params = params::Params::Empty;

    let map = |(organisation_id, abbreviation, name)| Organisation {
        id: organisation_id,
        abbreviation,
        name,
    };

    let terms = conn.exec_map(&stmt, &params, &map)?;
    Ok(terms)
}

pub fn organisation_create(conn: &mut PooledConn, organisation: &Organisation) -> Result<u32, Error> {
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
) -> Result<(), Error> {
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

pub fn organisation_delete(conn: &mut PooledConn, organisation_id: u32) -> Result<(), Error> {
    let stmt = conn.prep("DELETE s FROM organisations s WHERE s.organisation_id = :organisation_id")?;

    let params = params! {
        "organisation_id" => organisation_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
