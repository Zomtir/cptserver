use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Right, Team};
use crate::error::Error;

/*
 * METHODS
 */

pub fn team_list(conn: &mut PooledConn) -> Result<Vec<Team>, Error> {
    let stmt = conn.prep(
        "SELECT
            team_id,
            team_key,
            name,
            description        
        FROM teams;",
    )?;

    let params = params::Params::Empty;

    let rows: Vec<mysql::Row> = conn.exec(&stmt, &params)?;

    let mut teams: Vec<Team> = Vec::new();

    for mut row in rows {
        let team = Team {
            id: row.take("team_id").unwrap(),
            key: row.take("team_key").unwrap(),
            name: row.take("name").unwrap(),
            description: row.take("description").unwrap(),
            right: None,
        };
        teams.push(team);
    }

    Ok(teams)
}

pub fn team_info(conn: &mut PooledConn, team_id: &u32) -> Result<Team, Error> {
    let stmt = conn.prep(
        "SELECT
            team_id,
            team_key,
            name,
            description,
            right_club_write,
            right_club_read,
            right_competence_write,
            right_competence_read,
            right_course_write,
            right_course_read,
            right_event_write,
            right_event_read,
            right_inventory_write,
            right_inventory_read,
            right_location_write,
            right_location_read,
            right_organisation_write,
            right_organisation_read,
            right_team_write,
            right_team_read,
            right_user_write,
            right_user_read            
        FROM teams
        WHERE team_id = :team_id;",
    )?;

    let params = params! {
        "team_id" => team_id,
    };

    let mut row: mysql::Row = match conn.exec_first(&stmt, &params)? {
        None => return Err(Error::TeamMissing),
        Some(row) => row,
    };

    let team = Team {
        id: row.take("team_id").unwrap(),
        key: row.take("team_key").unwrap(),
        name: row.take("name").unwrap(),
        description: row.take("description").unwrap(),
        right: Some(Right {
            right_club_write: row.take("right_club_write").unwrap(),
            right_club_read: row.take("right_club_read").unwrap(),
            right_competence_write: row.take("right_competence_write").unwrap(),
            right_competence_read: row.take("right_competence_read").unwrap(),
            right_course_write: row.take("right_course_write").unwrap(),
            right_course_read: row.take("right_course_read").unwrap(),
            right_event_write: row.take("right_event_write").unwrap(),
            right_event_read: row.take("right_event_read").unwrap(),
            right_inventory_write: row.take("right_inventory_write").unwrap(),
            right_inventory_read: row.take("right_inventory_read").unwrap(),
            right_location_write: row.take("right_location_write").unwrap(),
            right_location_read: row.take("right_location_read").unwrap(),
            right_organisation_write: row.take("right_organisation_write").unwrap(),
            right_organisation_read: row.take("right_organisation_read").unwrap(),
            right_team_write: row.take("right_team_write").unwrap(),
            right_team_read: row.take("right_team_read").unwrap(),
            right_user_write: row.take("right_user_write").unwrap(),
            right_user_read: row.take("right_user_read").unwrap(),
        }),
    };

    Ok(team)
}

pub fn team_create(conn: &mut PooledConn, team: &Team) -> Result<u32, Error> {
    let stmt = conn.prep(
        "INSERT INTO teams (
            team_key,
            name,
            description)
        VALUES (
            :team_key,
            :name,
            :description);",
    )?;

    let params = params! {
        "team_key" => &team.key,
        "name" => &team.name,
        "description" => &team.description,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn team_edit(conn: &mut PooledConn, team_id: &u32, team: &Team) -> Result<(), Error> {
    let stmt = conn.prep(
        "UPDATE teams SET
            team_key = :team_key,
            name = :name,
            description = :description
        WHERE team_id = :team_id",
    )?;

    let params = params! {
        "team_id" => team_id,
        "team_key" => &team.key,
        "name" => &team.name,
        "description" => &team.description,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn team_right_edit(conn: &mut PooledConn, team_id: &u32, right: &Right) -> Result<(), Error> {
    let stmt = conn.prep(
        "UPDATE teams SET
            right_club_write = :right_club_write,
            right_club_read = :right_club_read,
            right_competence_write = :right_competence_write,
            right_competence_read = :right_competence_read,
            right_course_write = :right_course_write,
            right_course_read = :right_course_read,
            right_event_write = :right_event_write,
            right_event_read = :right_event_read,
            right_inventory_write = :right_inventory_write,
            right_inventory_read = :right_inventory_read,
            right_location_write = :right_location_write,
            right_location_read = :right_location_read,
            right_organisation_write = :right_organisation_write,
            right_organisation_read = :right_organisation_read,
            right_team_write = :right_team_write,
            right_team_read = :right_team_read,
            right_user_write = :right_user_write,
            right_user_read = :right_user_read
        WHERE team_id = :team_id",
    )?;

    let params = params! {
        "team_id" => team_id,
        "right_club_write" => &right.right_club_write,
        "right_club_read" => &right.right_club_read,
        "right_competence_write" => &right.right_competence_write,
        "right_competence_read" => &right.right_competence_read,
        "right_course_write" => &right.right_course_write,
        "right_course_read" => &right.right_course_read,
        "right_event_write" => &right.right_event_write,
        "right_event_read" => &right.right_event_read,
        "right_inventory_write" => &right.right_inventory_write,
        "right_inventory_read" => &right.right_inventory_read,
        "right_location_write" => &right.right_location_write,
        "right_location_read" => &right.right_location_read,
        "right_organisation_write" => &right.right_organisation_write,
        "right_organisation_read" => &right.right_organisation_read,
        "right_team_write" => &right.right_team_write,
        "right_team_read" => &right.right_team_read,
        "right_user_write" => &right.right_user_write,
        "right_user_read" => &right.right_user_read,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn team_delete(conn: &mut PooledConn, team_id: &u32) -> Result<(), Error> {
    let stmt = conn.prep("DELETE t FROM teams t WHERE t.team_id = :team_id")?;
    let params = params! {"team_id" => team_id};

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
