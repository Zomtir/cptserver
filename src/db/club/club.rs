use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::Club;
use crate::error::ErrorKind;

pub fn club_list(conn: &mut PooledConn) -> Result<Vec<Club>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT club_id, club_key, name
        FROM clubs;",
    )?;

    let params = params::Params::Empty;

    let map = |(club_id, club_key, club_name)| Club::from_info(club_id, club_key, club_name);

    let entries = conn.exec_map(&stmt, &params, &map)?;
    Ok(entries)
}

pub fn club_info(conn: &mut PooledConn, club_id: u32) -> Result<Club, ErrorKind> {
    let stmt = conn.prep(
        "SELECT club_id, club_key, name, description, disciplines, image_url, banner_url, chairman
        FROM clubs
        WHERE club_id = :club_id;",
    )?;

    let params = params! {
        "club_id" => &club_id,
    };

    let map = |(club_id, club_key, club_name, description, disciplines, image_url, banner_url, chairman)| Club {
        id: club_id,
        key: club_key,
        name: club_name,
        description,
        disciplines,
        image_url,
        banner_url,
        chairman,
    };

    let mut entries = conn.exec_map(&stmt, &params, &map)?;
    if entries.is_empty() {
        return Err(ErrorKind::ClubMissing);
    }
    Ok(entries.remove(0))
}

pub fn club_create(conn: &mut PooledConn, club: &Club) -> Result<u32, ErrorKind> {
    if let Some(image_url) = &club.image_url {
        crate::common::fs::validate_path(image_url)?;
    }
    if let Some(banner_url) = &club.banner_url {
        crate::common::fs::validate_path(banner_url)?;
    }

    let stmt = conn.prep(
        "INSERT INTO clubs (club_key, name, description, disciplines, image_url, banner_url, chairman)
        VALUES (:club_key, :name, :description, :disciplines, :image_url, :banner_url, :chairman)",
    )?;

    let params = params! {
        "club_key" => &club.key,
        "name" => &club.name,
        "description" => &club.description,
        "disciplines" => &club.disciplines,
        "image_url" => &club.image_url,
        "banner_url" => &club.banner_url,
        "chairman" => &club.chairman,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn club_edit(conn: &mut PooledConn, club_id: u32, club: &Club) -> Result<(), ErrorKind> {
    if let Some(image_url) = &club.image_url {
        crate::common::fs::validate_path(image_url)?;
    }
    if let Some(banner_url) = &club.banner_url {
        crate::common::fs::validate_path(banner_url)?;
    }

    let stmt = conn.prep(
        "UPDATE clubs SET
            club_key = :club_key,
            name = :name,
            description = :description,
            disciplines = :disciplines,
            image_url = :image_url,
            banner_url = :banner_url,
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
        "banner_url" => &club.banner_url,
        "chairman" => &club.chairman,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn club_delete(conn: &mut PooledConn, club_id: u32) -> Result<(), ErrorKind> {
    let stmt = conn.prep("DELETE c FROM clubs c WHERE c.club_id = :club_id")?;

    let params = params! {
        "club_id" => club_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
