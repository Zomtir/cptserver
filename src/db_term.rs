use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Club, Term, User};
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn term_list(
    club_id: Option<u32>,
    user_id: Option<u32>,
    point_in_time: Option<chrono::NaiveDate>,
) -> Result<Vec<Term>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT t.term_id,
            u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
            c.club_id, c.club_key, c.name, c.description,
            t.term_begin, t.term_end
        FROM terms t
        JOIN users u ON (u.user_id = t.user_id)
        JOIN clubs c ON (c.club_id = t.club_id)
        WHERE (:club_id IS NULL OR :club_id = t.club_id)
        AND (:user_id IS NULL OR :user_id = t.user_id)
        AND (:point_in_time IS NULL OR (:point_in_time BETWEEN t.term_begin AND t.term_end));",
    )?;

    let params = params! {
        "club_id" => club_id,
        "user_id" => user_id,
        "point_in_time" => point_in_time,
    };

    let map = |(
        term_id,
        user_id,
        user_key,
        firstname,
        lastname,
        nickname,
        club_id,
        club_key,
        club_name,
        club_description,
        begin,
        end,
    )| Term {
        id: term_id,
        user: User::from_info(user_id, user_key, firstname, lastname, nickname),
        club: Club {
            id: club_id,
            key: club_key,
            name: club_name,
            description: club_description,
        },
        begin,
        end,
    };

    let terms = conn.exec_map(&stmt, &params, &map)?;
    Ok(terms)
}

pub fn term_create(term: &Term) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO terms (user_id, club_id, term_begin, term_end)
        VALUES (:user_id, :club_id, :begin, :end)",
    )?;
    let params = params! {
        "user_id" => term.user.id,
        "club_id" => term.club.id,
        "begin" => &term.begin,
        "end" => &term.end,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn term_edit(term_id: i64, term: &Term) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE terms SET
            user_id  = :user_id,
            club_id = :club_id,
            term_begin = :begin,
            term_end = :end
        WHERE term_id = :term_id",
    )?;

    let params = params! {
        "term_id" => &term_id,
        "user_id" => &term.user.id,
        "club_id" => term.club.id,
        "begin" => &term.begin,
        "end" => &term.end,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn term_delete(term_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE t FROM terms t WHERE t.term_id = :term_id")?;

    let params = params! {
        "term_id" => term_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
