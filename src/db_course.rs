use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::db::get_pool_conn;
use crate::common::{User};

/*
 * METHODS
 */

pub fn get_course_moderator_list(course_id: &u32) -> Option<Vec<User>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT u.user_id, u.user_key, u.firstname, u.lastname
                          FROM users u
                          JOIN course_moderators m ON m.user_id = u.user_id
                          WHERE m.course_id = :course_id").unwrap();

    let params = params! { "course_id" => course_id};
    let map = |(user_id, user_key, firstname, lastname)| {
        User::from_info(user_id, user_key, firstname, lastname)
    };

    match conn.exec_map(&stmt, &params, &map) {
        Err(..) => None,
        Ok(members) => Some(members),
    }
}

pub fn is_course_moderator(course_id : & u32, user_id : & u32) -> Option<bool> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT COUNT(1)
                          FROM course_moderators
                          WHERE course_id = :course_id AND user_id = :user_id").unwrap();

    let params = params! {
        "course_id" => course_id,
        "user_id" => user_id,
    };

    match conn.exec_first::<u32,_,_>(&stmt, &params){
        Err(..) => return None,
        Ok(None) => return Some(false),
        Ok(Some(count)) => return Some(count == 1),
    };
}