use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::Team;
use crate::error::Error;

pub fn sieve_list(conn: &mut PooledConn, course_id: u32) -> Result<Vec<(Team, bool)>, Error> {
    let stmt = conn.prep(
        "SELECT t.team_id, t.team_key, t.name, t.description, cs.access
        FROM course_participant_sieves cs
        LEFT JOIN teams t ON cs.team_id = t.team_id
        WHERE course_id = :course_id;",
    )?;
    let params = params! {
        "course_id" => course_id,
    };
    let map = |(team_id, team_key, name, description, access)| {
        (
            Team {
                id: team_id,
                key: team_key,
                name,
                description,
                right: None,
            },
            access,
        )
    };

    let teams = conn.exec_map(&stmt, &params, &map)?;
    Ok(teams)
}

pub fn sieve_edit(conn: &mut PooledConn, course_id: u32, team_id: u64, access: bool) -> Result<(), Error> {
    let stmt = conn.prep(
        "INSERT INTO course_participant_sieves (course_id, team_id, access)
        VALUES (:course_id, :team_id, :access)
        ON DUPLICATE KEY UPDATE access = :access;",
    )?;
    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
        "access" => &access,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn sieve_remove(conn: &mut PooledConn, course_id: u32, team_id: u64) -> Result<(), Error> {
    let stmt = conn.prep(
        "DELETE FROM course_participant_sieves
        WHERE course_id = :course_id AND team_id = :team_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
