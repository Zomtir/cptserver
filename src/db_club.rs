use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Club, User};
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn club_list() -> Result<Vec<Club>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT club_id, club_key, name, description
        FROM clubs;",
    )?;

    let params = params::Params::Empty;

    let map = |(club_id, club_key, club_name, club_description)| Club {
        id: club_id,
        key: club_key,
        name: club_name,
        description: club_description,
    };

    let terms = conn.exec_map(&stmt, &params, &map)?;
    Ok(terms)
}

pub fn club_create(club: &Club) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO clubs (club_key, name, description)
        VALUES (:club_key, :name, :description)",
    )?;

    let params = params! {
        "club_key" => &club.key,
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
            club_key = :club_key,
            name = :name,
            description = :description
        WHERE club_id = :club_id",
    )?;

    let params = params! {
        "club_id" => &club_id,
        "club_key" => &club.key,
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

/* HOUSEKEEPING */

pub fn club_team_comparison(club_id: u32, team_id: u32, point_in_time: chrono::NaiveDate) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM users u
        INNER JOIN (
            SELECT tm.user_id
            FROM team_members tm
            WHERE tm.team_id = :team_id
            EXCEPT
            SELECT t.user_id
            FROM terms t
            WHERE t.club_id = :club_id
            AND (:point_in_time BETWEEN COALESCE(t.term_begin, :point_in_time) AND COALESCE(t.term_end, :point_in_time))
        ) AS missing ON missing.user_id = u.user_id;",
    )?;

    let params = params! {
        "club_id" => club_id,
        "team_id" => team_id,
        "point_in_time" => point_in_time,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let nonmembers = conn.exec_map(&stmt, &params, &map)?;
    Ok(nonmembers)
}

pub fn club_member_leaderboard(
    club_id: u32,
    active: Option<bool>,
    point_in_time: chrono::NaiveDate,
) -> Result<Vec<(User, u32)>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
            SUM(DATEDIFF(
                COALESCE(t.term_end, :point_in_time),
                COALESCE(t.term_begin, DATE_SUB(COALESCE(t.term_end, :point_in_time), INTERVAL 1 YEAR)))
            ) AS active_days
        FROM terms t
        JOIN users u ON u.user_id = t.user_id
        WHERE t.club_id = :club_id
        AND (:point_in_time <= COALESCE(t.term_end, :point_in_time))
        AND (:active IS NULL OR :active = u.active)
        GROUP BY u.user_id
        ORDER BY active_days DESC;",
    )?;

    let params = params! {
        "active" => &active,
        "club_id" => &club_id,
        "point_in_time" => point_in_time,
    };

    let map = |(user_id, user_key, firstname, lastname, nickname, active_days)| {
        (
            User::from_info(user_id, user_key, firstname, lastname, nickname),
            active_days,
        )
    };

    let leaderboard = conn.exec_map(&stmt, &params, &map)?;
    Ok(leaderboard)
}
