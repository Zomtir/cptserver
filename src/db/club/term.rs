use chrono::NaiveDate;
use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Club, Term, User};
use crate::error::ErrorKind;

fn row_map(
    (term_id, user_id, user_key, firstname, lastname, nickname, club_id, club_key, club_name, begin, end): (
        u64,
        u64,
        String,
        String,
        String,
        Option<String>,
        u64,
        String,
        String,
        Option<NaiveDate>,
        Option<NaiveDate>,
    ),
) -> Term {
    Term {
        id: term_id,
        user: User::from_info(user_id, user_key, firstname, lastname, nickname),
        club: Club::from_info(club_id, club_key, club_name),
        begin,
        end,
    }
}

pub fn term_list(
    conn: &mut PooledConn,
    club_id: Option<u32>,
    user_id: Option<u32>,
    point_in_time: Option<chrono::NaiveDate>,
) -> Result<Vec<Term>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT t.term_id,
            u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
            c.club_id, c.club_key, c.name,
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

    let terms = conn.exec_map(&stmt, &params, &row_map)?;
    Ok(terms)
}

pub fn term_info(conn: &mut PooledConn, term_id: u32) -> Result<Term, ErrorKind> {
    let stmt = conn.prep(
        "SELECT t.term_id,
            u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
            c.club_id, c.club_key, c.name,
            t.term_begin, t.term_end
        FROM terms t
        JOIN users u ON (u.user_id = t.user_id)
        JOIN clubs c ON (c.club_id = t.club_id)
        WHERE :term_id = t.term_id;",
    )?;

    let params = params! {
        "term_id" => term_id,
    };

    let row = conn.exec_first(&stmt, &params)?;
    row.map(row_map).ok_or(ErrorKind::Missing)
}

pub fn term_create(conn: &mut PooledConn, term: &Term) -> Result<u32, ErrorKind> {
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

pub fn term_edit(conn: &mut PooledConn, term_id: i64, term: &Term) -> Result<(), ErrorKind> {
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

pub fn term_delete(conn: &mut PooledConn, term_id: i64) -> Result<(), ErrorKind> {
    let stmt = conn.prep("DELETE t FROM terms t WHERE t.term_id = :term_id")?;

    let params = params! {
        "term_id" => term_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
