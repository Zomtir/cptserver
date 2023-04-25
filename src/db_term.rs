use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::db::get_pool_conn;
use crate::common::{Term, User};
use crate::error::Error;

pub fn list_terms(user_id: Option<i64>) -> Option<Vec<Term>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT t.term_id,
            u.user_id, u.user_key, u.firstname, u.lastname,
            t.term_begin, t.term_end
        FROM terms t
        JOIN users u ON (t.user_id = u.user_id)
        WHERE (:user_id IS NULL OR :user_id = t.user_id);");

    let params = params! {
        "user_id" => user_id,
    };

    let map = |(term_id,
        user_id, user_key, firstname, lastname,
        begin, end)|
    Term {
        id: term_id,
        user: User::from_info(user_id, user_key, firstname, lastname),
        begin,
        end};

    match conn.exec_map(&stmt.unwrap(),&params,&map) {
        Err(..) => None,
        Ok(terms) => Some(terms),
    }
}

pub fn create_term(term: &Term) -> Result<u32, Error> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO terms (user_id, begin, end)
        VALUES (:user_id, :begin, :end)");
    let params = params! {
        "user_id" => term.user.id,
        "begin" => &term.begin,
        "end" => &term.end,
    };

    conn.exec_drop(&stmt.unwrap(),&params)?;
    
    crate::db::get_last_id(&mut conn)
}

pub fn edit_term(term_id: i64, term: &Term) -> Option<()> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("
        UPDATE terms SET
            user_id  = :user_id,
            begin = :begin,
            end = :end,
        WHERE term_id = :term_id").unwrap();

    let params = params! {
        "term_id" => &term_id,
        "user_id" => &term.user.id,
        "begin" => &term.begin,
        "end" => &term.end,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn delete_term(term_id: i64) -> Option<()> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE t FROM terms t WHERE t.term_id_id = :term_id");

    let params = params! {
        "term_id" => term_id
    };

    match conn.exec_drop(&stmt.unwrap(),&params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn get_user_membership_days(enabled: Option<bool>) -> Option<Vec<(i64,i64)>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT t.user_id, SUM(DATEDIFF(t.term_end, t.term_begin)) as active_days 
        FROM terms t
        JOIN users u ON u.user_id = t.user_id
        WHERE (:enabled IS NULL OR :enabled = u.enabled)
        GROUP BY t.user_id
        ORDER BY active_days DESC;");

    let params = params! {
        "enabled" => &enabled,
    };

    let map = |(user_id, active_days) : (i64, i64)| {
        (user_id, active_days)
    };
    
    match conn.exec_map(&stmt.unwrap(), &params, &map) {
        Err(..) => None,
        Ok(leaderboard) => Some(leaderboard),
    }
}

pub fn get_wrong_enabled_users() -> Option<Vec<User>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname
        FROM users u
        LEFT JOIN
        (
            SELECT t.user_id as user_id, TRUE as valid
            FROM terms t
            WHERE IFNULL(t.term_begin,'0000-01-01') < UTC_DATE()
            AND IFNULL(t.term_end,'9999-12-31') > UTC_DATE()
            GROUP BY t.user_id
        ) AS termstatus ON u.user_id = termstatus.user_id
        WHERE u.enabled = TRUE
        AND termstatus.valid IS NULL;");

    let params = params::Params::Empty;

    let map = |(user_id, user_key, firstname, lastname)| {
        User::from_info(user_id, user_key, firstname, lastname)
    };
    
    match conn.exec_map(&stmt.unwrap(), &params, &map) {
        Err(..) => None,
        Ok(users) => Some(users),
    }
}

pub fn get_wrong_disabled_users() -> Option<Vec<User>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname
        FROM users u
        LEFT JOIN
        (
            SELECT t.user_id as user_id, TRUE as valid
            FROM terms t
            WHERE IFNULL(t.term_begin,'0000-01-01') < UTC_DATE()
            AND IFNULL(t.term_end,'9999-12-31') > UTC_DATE()
            GROUP BY t.user_id
        ) AS termstatus ON u.user_id = termstatus.user_id
        WHERE u.enabled = FALSE
        AND termstatus.valid = TRUE;");

    let params = params::Params::Empty;

    let map = |(user_id, user_key, firstname, lastname)| {
        User::from_info(user_id, user_key, firstname, lastname)
    };
    
    match conn.exec_map(&stmt.unwrap(), &params, &map) {
        Err(..) => None,
        Ok(users) => Some(users),
    }
}
