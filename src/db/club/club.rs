use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::Club;
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn club_list() -> Result<Vec<Club>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT club_id, club_key, name
        FROM clubs;",
    )?;

    let params = params::Params::Empty;

    let map = |(club_id, club_key, club_name)| Club::from_info(club_id, club_key, club_name);

    let entries = conn.exec_map(&stmt, &params, &map)?;
    Ok(entries)
}

pub fn club_info(club_id: u32) -> Result<Club, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT club_id, club_key, name, description, disciplines, image_url, chairman
        FROM clubs
        WHERE club_id = :club_id;",
    )?;

    let params = params! {
        "club_id" => &club_id,
    };

    let map = |(club_id, club_key, club_name, description, disciplines, image_url, chairman)| Club {
        id: club_id,
        key: club_key,
        name: club_name,
        description,
        disciplines,
        image_url,
        chairman,
    };

    let mut entries = conn.exec_map(&stmt, &params, &map)?;
    if entries.len() < 1 {
        return Err(Error::ClubMissing);
    }
    Ok(entries.remove(0))
}

pub fn club_create(club: &Club) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO clubs (club_key, name, description, disciplines, image_url, chairman)
        VALUES (:club_key, :name, :description, :disciplines, :image_url, :chairman)",
    )?;

    let params = params! {
        "club_key" => &club.key,
        "name" => &club.name,
        "description" => &club.description,
        "disciplines" => &club.disciplines,
        "image_url" => &club.image_url,
        "chairman" => &club.chairman,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn club_edit(club_id: u32, club: &Club) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE clubs SET
            club_key = :club_key,
            name = :name,
            description = :description,
            disciplines = :disciplines,
            image_url = :image_url,
            chairman = :chairman
        WHERE club_id = :club_id",
    )?;

    let params = params! {
        "club_id" => &club_id,
        "club_key" => &club.key,
        "name" => &club.name,
        "description" => &club.description,
        "disciplines" => &club.disciplines,
        "image_url" => &club.image_url,
        "chairman" => &club.chairman,
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
