use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::db::get_pool_conn;

/*
 * METHODS
 */

pub fn is_user_created(user_key: & str) -> Option<bool> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT COUNT(1) FROM users WHERE user_key = :user_key").ok()?;
    let count : Option<i32> = conn.exec_first(&stmt, params! { "user_key" => user_key }).ok()?;

    return Some(count.unwrap() == 1);
}
