use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::User;
use crate::error::ErrorKind;

pub fn team_member_list(conn: &mut PooledConn, team_id: u32) -> Result<Vec<User>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM users u
        JOIN team_members m ON m.user_id = u.user_id
        WHERE m.team_id = :team_id",
    )?;

    let params = params! {
        "team_id" => team_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let members = conn.exec_map(&stmt, &params, &map)?;
    Ok(members)
}

pub fn team_member_add(conn: &mut PooledConn, team_id: &u32, user_id: &u32) -> Result<(), ErrorKind> {
    let stmt = conn.prep("INSERT INTO team_members (team_id, user_id) SELECT :team_id, :user_id")?;
    let params = params! {
        "team_id" => &team_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn team_member_remove(conn: &mut PooledConn, team_id: &u32, user_id: &u32) -> Result<(), ErrorKind> {
    let stmt = conn.prep("DELETE FROM team_members WHERE team_id = :team_id AND user_id = :user_id")?;
    let params = params! {
        "team_id" => &team_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
