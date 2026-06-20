use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::Term;
use crate::error::{Error, ErrorKind, Result};

pub fn term_list(
    conn: &mut PooledConn,
    club_id: Option<u32>,
    user_id: Option<u32>,
    point_in_time: Option<chrono::NaiveDate>,
) -> Result<Vec<Term>> {
    let stmt = conn.prep(
        "SELECT t.term_id,
            u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
            c.club_id, c.club_key, c.name AS club_name,
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

    let rows: Vec<mysql::Row> = conn.exec(&stmt, &params)?;

    let mut terms = Vec::new();

    for mut row in rows {
        let term = Term::from_row(
            row.take("term_id"),
            row.take("user_id"),
            row.take("user_key"),
            row.take("firstname"),
            row.take("lastname"),
            row.take("nickname"),
            row.take("club_id"),
            row.take("club_key"),
            row.take("club_name"),
            row.take("term_begin"),
            row.take("term_end"),
        );

        terms.push(term.unwrap());
    }

    Ok(terms)
}

pub fn term_info(conn: &mut PooledConn, term_id: u64) -> Result<Term> {
    let stmt = conn.prep(
        "SELECT t.term_id,
            u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
            c.club_id, c.club_key, c.name As club_name,
            t.term_begin, t.term_end
        FROM terms t
        JOIN users u ON (u.user_id = t.user_id)
        JOIN clubs c ON (c.club_id = t.club_id)
        WHERE :term_id = t.term_id;",
    )?;

    let params = params! {
        "term_id" => term_id,
    };

    let mut row = conn
        .exec_first::<mysql::Row, _, _>(&stmt, &params)?
        .ok_or(Error::new(ErrorKind::Missing, "Term not found"))?;

    let mut term = Term::from_row(
        row.take("term_id"),
        row.take("user_id"),
        row.take("user_key"),
        row.take("firstname"),
        row.take("lastname"),
        row.take("nickname"),
        row.take("club_id"),
        row.take("club_key"),
        row.take("club_name"),
        row.take("term_begin"),
        row.take("term_end"),
    )
    .unwrap();

    term.disciplines = crate::db::club::term_discipline::term_discipline_list(conn, Some(term.id)).ok();

    Ok(term)
}

pub fn term_create(conn: &mut PooledConn, term: &Term) -> Result<u32> {
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

pub fn term_edit(conn: &mut PooledConn, term_id: u32, term: &Term) -> Result<()> {
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

pub fn term_delete(conn: &mut PooledConn, term_id: u32) -> Result<()> {
    let stmt = conn.prep("DELETE t FROM terms t WHERE t.term_id = :term_id")?;

    let params = params! {
        "term_id" => term_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
