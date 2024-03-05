use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::Club;
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn club_list() -> Result<Vec<Club>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT c.club_id, c.name, c.description
        FROM clubs c;",
    )?;

    let params = params::Params::Empty;

    let map = |(club_id, club_name, club_description)| Club {
        id: club_id,
        name: club_name,
        description: club_description,
    };

    let terms = conn.exec_map(&stmt, &params, &map)?;
    Ok(terms)
}

pub fn club_create(club: &Club) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO clubs (club_id, name, description)
        VALUES (:club_id, :name, :description)",
    )?;

    let params = params! {
        "user_id" => &club.id,
        "name" => &club.name,
        "description" => &club.description,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn club_edit(club_id: u32, club: &Club) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE clubs SET
            name = :name,
            description = :description,
        WHERE club_id = :club_id",
    )?;

    let params = params! {
        "club_id" => &club_id,
        "name" => &club.name,
        "description" => &club.description,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn club_delete(club_id: u32) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE c FROM clubs c WHERE c.club_id = :club_id")?;

    let params = params! {
        "club_id" => club_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
