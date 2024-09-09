use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Affiliation, Organisation, User};
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn sql_affiliation(mut row: mysql::Row) -> Affiliation {
    Affiliation {
        user: row.take::<Option<u64>, &str>("user_id").unwrap().map(|user_id| {
            User::from_info(
                user_id,
                row.take("user_key").unwrap(),
                row.take("firstname").unwrap(),
                row.take("lastname").unwrap(),
                row.take("nickname").unwrap(),
            )
        }),
        organisation: row
            .take::<Option<u64>, &str>("organisation_id")
            .unwrap()
            .map(|organisation_id| Organisation {
                id: organisation_id,
                abbreviation: row.take("organisation_abbreviation").unwrap(),
                name: row.take("organisation_name").unwrap(),
            }),
        member_identifier: row.take("member_identifier").unwrap(),
        permission_solo_date: row.take("permission_solo_date").unwrap(),
        permission_team_date: row.take("permission_team_date").unwrap(),
        residency_move_date: row.take("residency_move_date").unwrap(),
    }
}

pub fn affiliation_list(user_id: Option<u64>, organisation_id: Option<u64>) -> Result<Vec<Affiliation>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
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

    for row in rows {
        affiliations.push(crate::db::organisation::sql_affiliation(row));
    }

    Ok(affiliations)
}

pub fn affiliation_info(user_id: u64, organisation_id: u64) -> Result<Option<Affiliation>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
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

    match conn.exec_first(&stmt, &params)? {
        None => Ok(None),
        Some(row) => Ok(Some(sql_affiliation(row))),
    }
}

pub fn affiliation_create(user_id: u64, organisation_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
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

pub fn affiliation_edit(user_id: u64, organisation_id: u64, affiliation: &Affiliation) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
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

pub fn affiliation_delete(user_id: u64, organisation_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
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
