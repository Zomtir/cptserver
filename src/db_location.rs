use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::Location;
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn location_list() -> Result<Vec<Location>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT location_id, location_key, name, description
        FROM locations;",
    )?;

    let params = params::Params::Empty;

    let map = |(location_id, location_key, name, description)| Location {
        id: location_id,
        key: location_key,
        name,
        description,
    };

    let terms = conn.exec_map(&stmt, &params, &map)?;
    Ok(terms)
}

pub fn location_create(location: &Location) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO locations (location_key, name, description)
        VALUES (:location_key, :name, :description)",
    )?;

    let params = params! {
        "location_key" => &location.key,
        "name" => &location.name,
        "description" => &location.description,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn location_edit(location_id: u32, location: &Location) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE locations SET
            location_key = :location_key,
            name = :name,
            description = :description
        WHERE location_id = :location_id",
    )?;

    let params = params! {
        "location_id" => &location_id,
        "location_key" => &location.key,
        "name" => &location.name,
        "description" => &location.description,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn location_delete(location_id: u32) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE s FROM locations s WHERE s.location_id = :location_id")?;

    let params = params! {
        "location_id" => location_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
