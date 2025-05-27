use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::Affiliation;
use crate::error::ErrorKind;

pub fn affiliation_list(
    conn: &mut PooledConn,
    user_id: Option<u64>,
    organisation_id: Option<u32>,
) -> Result<Vec<Affiliation>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT
            u.user_id, u.user_key,
            u.firstname AS user_firstname,
            u.lastname AS user_lastname,
            u.nickname AS user_nickname,
            o.organisation_id,
            o.abbreviation as organisation_abbreviation,
            o.name as organisation_name,
            oa.member_identifier,
            oa.permission_solo_date,
            oa.permission_team_date,
            oa.residency_move_date
        FROM organisation_affiliations oa
        LEFT JOIN users u ON u.user_id = oa.user_id
        LEFT JOIN organisations o ON o.organisation_id = oa.organisation_id
        WHERE (:user_id IS NULL OR oa.user_id = :user_id)
        AND (:organisation_id IS NULL OR oa.organisation_id = :organisation_id);",
    )?;

    let params = params! {
        "user_id" => user_id,
        "organisation_id" => organisation_id,
    };

    let rows: Vec<mysql::Row> = conn.exec(&stmt, &params)?;

    let mut affiliations: Vec<Affiliation> = Vec::new();

    for mut row in rows {
        affiliations.push(Affiliation::from_row(&mut row));
    }

    Ok(affiliations)
}

pub fn affiliation_info(
    conn: &mut PooledConn,
    user_id: u64,
    organisation_id: u32,
) -> Result<Option<Affiliation>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT
            u.user_id, u.user_key, u.firstname AS user_firstname, u.lastname AS user_lastname, u.nickname AS user_nickname,
            o.organisation_id,
            o.abbreviation as organisation_abbreviation,
            o.name as organisation_name,
            oa.member_identifier,
            oa.permission_solo_date,
            oa.permission_team_date,
            oa.residency_move_date
        FROM organisation_affiliations oa
        LEFT JOIN users u ON u.user_id = oa.user_id
        LEFT JOIN organisations o ON o.organisation_id = oa.organisation_id
        WHERE oa.user_id = :user_id AND oa.organisation_id = :organisation_id;",
    )?;

    let params = params! {
        "user_id" => &user_id,
        "organisation_id" => &organisation_id,
    };

    let row = conn.exec_first(&stmt, &params)?;

    match row {
        None => Ok(None),
        Some(mut row) => Ok(Some(Affiliation::from_row(&mut row))),
    }
}

pub fn affiliation_create(conn: &mut PooledConn, user_id: u64, organisation_id: u32) -> Result<(), ErrorKind> {
    let stmt = conn.prep(
        "INSERT INTO organisation_affiliations (user_id, organisation_id)
        SELECT :user_id, :organisation_id;",
    )?;

    let params = params! {
        "user_id" => user_id,
        "organisation_id" => &organisation_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn affiliation_edit(
    conn: &mut PooledConn,
    user_id: u64,
    organisation_id: u32,
    affiliation: &Affiliation,
) -> Result<(), ErrorKind> {
    let stmt = conn.prep(
        "UPDATE organisation_affiliations SET
        member_identifier = :member_identifier,
        permission_solo_date = :permission_solo_date,
        permission_team_date = :permission_team_date,
        residency_move_date = :residency_move_date
        WHERE user_id = :user_id AND organisation_id = :organisation_id;",
    )?;

    let params = params! {
        "user_id" => user_id,
        "organisation_id" => &organisation_id,
        "member_identifier" => &affiliation.member_identifier,
        "permission_solo_date" => &affiliation.permission_solo_date,
        "permission_team_date" => &affiliation.permission_team_date,
        "residency_move_date" => &affiliation.residency_move_date,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn affiliation_delete(conn: &mut PooledConn, user_id: u64, organisation_id: u32) -> Result<(), ErrorKind> {
    let stmt = conn.prep(
        "DELETE FROM organisation_affiliations
        WHERE user_id = :user_id AND organisation_id = :organisation_id;",
    )?;

    let params = params! {
        "user_id" => user_id,
        "organisation_id" => organisation_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
