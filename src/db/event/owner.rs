use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::User;
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn event_owner_list(event_id: u64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM event_owners
        JOIN users u ON u.user_id = event_owners.user_id
        WHERE event_owners.event_id = :event_id",
    )?;
    let params = params! {
        "event_id" => event_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn event_owner_add(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();

    let stmt = conn.prep(
        "INSERT INTO event_owners (event_id, user_id)
        VALUES (:event_id, :user_id)",
    )?;
    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_owner_remove(event_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM event_owners
        WHERE event_id = :event_id AND user_id = :user_id",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_owner_true(event_id: u64, user_id: u64) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM event_owners
        WHERE event_id = :event_id AND user_id = :user_id",
    )?;
    let params = params! {
        "event_id" => event_id,
        "user_id" => user_id,
    };

    match conn.exec_first::<u64, _, _>(&stmt, &params)? {
        None => Ok(false),
        Some(count) => Ok(count == 1),
    }
}
