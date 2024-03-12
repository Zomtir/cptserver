use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Right, Team, User};
use crate::db::get_pool_conn;
use crate::error::Error;

/*
 * METHODS
 */

pub fn list_teams() -> Result<Vec<Team>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            team_id,
            name,
            description,
            admin_competence,
            admin_courses,
            admin_event,
            admin_inventory,
            admin_teams,
            admin_term,
            admin_users
        FROM teams",
    )?;
    let map = |(
        team_id,
        name,
        description,
        admin_competence,
        admin_courses,
        admin_event,
        admin_inventory,
        admin_teams,
        admin_term,
        admin_users,
    )| Team {
        id: team_id,
        name,
        description,
        right: Some(Right {
            admin_competence,
            admin_courses,
            admin_event,
            admin_inventory,
            admin_teams,
            admin_term,
            admin_users,
        }),
    };

    let params = params::Params::Empty;

    let teams = conn.exec_map(&stmt, &params, &map)?;
    Ok(teams)
}

pub fn create_team(team: &Team) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "INSERT INTO teams (
            name,
            description,
            admin_competence,
            admin_courses,
            admin_event,
            admin_inventory,
            admin_teams,
            admin_term,
            admin_users)
        VALUES (
            :name,
            :description,
            :admin_courses,
            :admin_event,
            :admin_inventory,
            :admin_competence,
            :admin_teams,
            :admin_term,
            :admin_users)",
    )?;

    let rights = match &team.right {
        None => return Err(Error::Default),
        Some(r) => r.clone(),
    };

    let params = params! {
        "name" => &team.name,
        "description" => &team.description,
        "admin_courses" => &rights.admin_courses,
        "admin_event" => &rights.admin_event,
        "admin_inventory" => &rights.admin_inventory,
        "admin_competence," => &rights.admin_competence,
        "admin_teams" => &rights.admin_teams,
        "admin_term" => &rights.admin_term,
        "admin_users" => &rights.admin_users,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn edit_team(team_id: &u32, team: &Team) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE teams SET
            name = :name,
            description = :description,
            admin_competence = :admin_competence,
            admin_courses = :admin_courses,
            admin_event = :admin_event,
            admin_inventory = :admin_inventory,
            admin_teams = :admin_teams,
            admin_term = :admin_term,
            admin_users = :admin_users
        WHERE team_id = :team_id",
    )?;

    let rights = match &team.right {
        None => return Err(Error::Default),
        Some(r) => r.clone(),
    };

    let params = params! {
        "team_id" => &team_id,
        "name" => &team.name,
        "description" => &team.description,
        "admin_competence" => &rights.admin_competence,
        "admin_courses" => &rights.admin_courses,
        "admin_event" => &rights.admin_event,
        "admin_inventory" => &rights.admin_inventory,
        "admin_teams" => &rights.admin_teams,
        "admin_term" => &rights.admin_term,
        "admin_users" => &rights.admin_users,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn delete_team(team_id: &u32) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE t FROM teams t WHERE t.team_id = :team_id")?;
    let params = params! {"team_id" => team_id};

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn list_team_members(team_id: u32) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM users u
        JOIN team_members m ON m.user_id = u.user_id
        WHERE m.team_id = :team_id",
    )?;

    let params = params! {
        "team_id" => team_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| User::from_info(user_id, user_key, firstname, lastname, nickname);

    let members = conn.exec_map(&stmt, &params, &map)?;
    Ok(members)
}

pub fn add_team_member(team_id: &u32, user_id: &u32) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO team_members (team_id, user_id) SELECT :team_id, :user_id")?;
    let params = params! {
        "team_id" => &team_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn remove_team_member(team_id: &u32, user_id: &u32) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE FROM team_members WHERE team_id = :team_id AND user_id = :user_id")?;
    let params = params! {
        "team_id" => &team_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
