use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::User;
use crate::error::Error;

pub fn course_moderator_list(conn: &mut PooledConn, course_id: u32) -> Result<Vec<User>, Error> {
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM users u
        JOIN course_moderators m ON m.user_id = u.user_id
        WHERE m.course_id = :course_id",
    )?;

    let params = params! {
        "course_id" => course_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let members = conn.exec_map(&stmt, &params, &map)?;
    Ok(members)
}

pub fn course_moderator_true(conn: &mut PooledConn, course_id: u32, user_id: u64) -> Result<bool, Error> {
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM course_moderators
        WHERE course_id = :course_id AND user_id = :user_id",
    )?;

    let params = params! {
        "course_id" => course_id,
        "user_id" => user_id,
    };

    match conn.exec_first::<u32, _, _>(&stmt, &params)? {
        None => Ok(false),
        Some(count) => Ok(count == 1),
    }
}

pub fn course_moderator_add(conn: &mut PooledConn, course_id: u32, user_id: u64) -> Result<(), Error> {
    let stmt = conn.prep(
        "INSERT INTO course_moderators (course_id, user_id)
                          SELECT :course_id, :user_id",
    )?;
    let params = params! {
        "course_id" => &course_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn course_moderator_remove(conn: &mut PooledConn, course_id: u32, user_id: u64) -> Result<(), Error> {
    let stmt = conn
        .prep(
            "DELETE e FROM course_moderators e
            WHERE course_id = :course_id AND user_id = :user_id",
        )
        .unwrap();
    let params = params! {
        "course_id" => &course_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
