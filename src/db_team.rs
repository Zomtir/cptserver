use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Right, Team, User};
use crate::db::get_pool_conn;
use crate::error::Error;

/*
 * METHODS
 */

pub fn list_teams() -> Option<Vec<Team>> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            team_id,
            name,
            description,
            admin_courses,
            admin_event,
            admin_inventory,
            admin_rankings,
            admin_teams,
            admin_term,
            admin_users
        FROM teams",
    );
    let map = |(
        team_id,
        name,
        description,
        admin_courses,
        admin_event,
        admin_inventory,
        admin_rankings,
        admin_teams,
        admin_term,
        admin_users,
    )| Team {
        id: team_id,
        name,
        description,
        right: Right {
            admin_courses,
            admin_event,
            admin_inventory,
            admin_rankings,
            admin_teams,
            admin_term,
            admin_users,
        },
    };

    let params = params::Params::Empty;

    match conn.exec_map(&stmt.unwrap(), &params, &map) {
        Err(..) => None,
        Ok(teams) => Some(teams),
    }
}

pub fn create_team(team: &Team) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "INSERT INTO teams (
            name,
            description,
            admin_courses,
            admin_event,
            admin_inventory,
            admin_rankings,
            admin_teams,
            admin_term,
            admin_users)
        VALUES (
            :name,
            :description,
            :admin_courses,
            :admin_event,
            :admin_inventory,
            :admin_rankings,
            :admin_teams,
            :admin_term,
            :admin_users)",
    );

    let params = params! {
        "name" => &team.name,
        "description" => &team.description,
        "admin_courses" => &team.right.admin_courses,
        "admin_event" => &team.right.admin_event,
        "admin_inventory" => &team.right.admin_inventory,
        "admin_rankings" => &team.right.admin_rankings,
        "admin_teams" => &team.right.admin_teams,
        "admin_term" => &team.right.admin_term,
        "admin_users" => &team.right.admin_users,
    };

    conn.exec_drop(&stmt.unwrap(), &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn edit_team(team_id: &u32, team: &Team) -> Option<()> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE teams SET
            name = :name,
            description = :description,
            admin_courses = :admin_courses,
            admin_event = :admin_event,
            admin_inventory = :admin_inventory,
            admin_rankings = :admin_rankings,
            admin_teams = :admin_teams,
            admin_term = :admin_term,
            admin_users = :admin_users
        WHERE team_id = :team_id",
    );

    let params = params! {
        "team_id" => &team_id,
        "name" => &team.name,
        "description" => &team.description,
        "admin_courses" => &team.right.admin_courses,
        "admin_event" => &team.right.admin_event,
        "admin_inventory" => &team.right.admin_inventory,
        "admin_rankings" => &team.right.admin_rankings,
        "admin_teams" => &team.right.admin_teams,
        "admin_term" => &team.right.admin_term,
        "admin_users" => &team.right.admin_users,
    };

    match conn.exec_drop(&stmt.unwrap(), &params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn delete_team(team_id: &u32) -> Option<()> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE t FROM teams t WHERE t.team_id = :team_id");
    let params = params! {"team_id" => team_id};

    match conn.exec_drop(&stmt.unwrap(), &params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn list_team_members(team_id: u32) -> Option<Vec<User>> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname
        FROM users u
        JOIN team_members m ON m.user_id = u.user_id
        WHERE m.team_id = :team_id",
    );

    let params = params! {
        "team_id" => team_id,
    };
    let map = |(user_id, user_key, firstname, lastname)| User::from_info(user_id, user_key, firstname, lastname);

    match conn.exec_map(&stmt.unwrap(), &params, &map) {
        Err(..) => None,
        Ok(members) => Some(members),
    }
}

pub fn add_team_member(team_id: &u32, user_id: &u32) -> Option<()> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO team_members (team_id, user_id) SELECT :team_id, :user_id");
    let params = params! {
        "team_id" => &team_id,
        "user_id" => &user_id,
    };

    match conn.exec_drop(&stmt.unwrap(), &params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn remove_team_member(team_id: &u32, user_id: &u32) -> Option<()> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE FROM team_members WHERE team_id = :team_id AND e.user_id = :user_id");
    let params = params! {
        "team_id" => &team_id,
        "user_id" => &user_id,
    };

    match conn.exec_drop(&stmt.unwrap(), &params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}
