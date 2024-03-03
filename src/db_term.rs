use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Club, Term, User};
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn list_terms(user_id: Option<i64>) -> Result<Vec<Term>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT t.term_id,
            u.user_id, u.user_key, u.firstname, u.lastname,
            c.club_id, c.name, c.description,
            t.term_begin, t.term_end
        FROM terms t
        JOIN users u ON (u.user_id = t.user_id)
        JOIN clubs c ON (c.club_id = t.club_id)
        WHERE (:user_id IS NULL OR :user_id = t.user_id);",
    )?;

    let params = params! {
        "user_id" => user_id,
    };

    let map =
        |(term_id, user_id, user_key, firstname, lastname, club_id, club_name, club_description, begin, end)| Term {
            id: term_id,
            user: User::from_info(user_id, user_key, firstname, lastname),
            club: Club {
                id: club_id,
                name: club_name,
                description: club_description,
            },
            begin,
            end,
        };

    let terms = conn.exec_map(&stmt, &params, &map)?;
    Ok(terms)
}

pub fn create_term(term: &Term) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO terms (user_id, term_begin, term_end)
        VALUES (:user_id, :begin, :end)",
    )?;
    let params = params! {
        "user_id" => term.user.id,
        "begin" => &term.begin,
        "end" => &term.end,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn edit_term(term_id: i64, term: &Term) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE terms SET
            user_id  = :user_id,
            term_begin = :begin,
            term_end = :end,
        WHERE term_id = :term_id",
    )?;

    let params = params! {
        "term_id" => &term_id,
        "user_id" => &term.user.id,
        "begin" => &term.begin,
        "end" => &term.end,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn delete_term(term_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE t FROM terms t WHERE t.term_id = :term_id")?;

    let params = params! {
        "term_id" => term_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn get_user_membership_days(active: Option<bool>) -> Result<Vec<(i64, i64)>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT t.user_id, SUM(DATEDIFF(t.term_end, t.term_begin)) as active_days 
        FROM terms t
        JOIN users u ON u.user_id = t.user_id
        WHERE (:active IS NULL OR :active = u.active)
        GROUP BY t.user_id
        ORDER BY active_days DESC;",
    )?;

    let params = params! {
        "active" => &active,
    };

    let map = |(user_id, active_days): (i64, i64)| (user_id, active_days);

    let leaderboard = conn.exec_map(&stmt, &params, &map)?;
    Ok(leaderboard)
}

pub fn get_wrong_active_users() -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
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
        WHERE u.active = TRUE
        AND termstatus.valid IS NULL;",
    )?;

    let params = params::Params::Empty;

    let map = |(user_id, user_key, firstname, lastname)| User::from_info(user_id, user_key, firstname, lastname);

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn get_wrong_inactive_users() -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
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
        WHERE u.active = FALSE
        AND termstatus.valid = TRUE;",
    )?;

    let params = params::Params::Empty;

    let map = |(user_id, user_key, firstname, lastname)| User::from_info(user_id, user_key, firstname, lastname);

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}
