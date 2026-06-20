use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::TermDiscipline;
use crate::error::Result;

pub fn term_discipline_list(conn: &mut PooledConn, term_id: Option<u64>) -> Result<Vec<TermDiscipline>> {
    let stmt = conn.prep(
        "SELECT td.term_discipline_id,
            d.discipline_id, d.name AS discipline_name,
            td.begin, td.end
        FROM term_disciplines td
        JOIN disciplines d ON (d.discipline_id = td.discipline_id)
        WHERE (:term_id IS NULL OR :term_id = td.term_id);",
    )?;

    let params = params! {
        "term_id" => term_id,
    };

    let rows: Vec<mysql::Row> = conn.exec(&stmt, &params)?;

    let mut term_disciplines: Vec<TermDiscipline> = Vec::new();

    for mut row in rows {
        let uc = TermDiscipline::from_row(
            row.take("term_discipline_id"),
            row.take("discipline_id"),
            row.take("discipline_name"),
            row.take("begin"),
            row.take("end"),
        );
        term_disciplines.push(uc.unwrap());
    }

    Ok(term_disciplines)
}

pub fn term_discipline_create(conn: &mut PooledConn, term_id: u32, term_discipline: &TermDiscipline) -> Result<u32> {
    let stmt = conn.prep(
        "INSERT INTO term_disciplines (term_id, discipline_id, begin, end)
        VALUES (:term_id, :discipline_id, :begin, :end)",
    )?;
    let params = params! {
        "term_id" => term_id,
        "discipline_id" => term_discipline.discipline.id,
        "begin" => &term_discipline.begin,
        "end" => &term_discipline.end,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn term_discipline_edit(
    conn: &mut PooledConn,
    term_discipline_id: u32,
    term_discipline: &TermDiscipline,
) -> Result<()> {
    let stmt = conn.prep(
        "UPDATE term_disciplines SET
            discipline_id = :discipline_id,
            begin = :begin,
            end = :end
        WHERE term_discipline_id = :term_discipline_id",
    )?;

    let params = params! {
        "term_discipline_id" => term_discipline_id,
        "discipline_id" => &term_discipline.discipline.id,
        "begin" => &term_discipline.begin,
        "end" => &term_discipline.end,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn term_discipline_delete(conn: &mut PooledConn, term_discipline_id: u32) -> Result<()> {
    let stmt = conn.prep("DELETE FROM term_disciplines WHERE term_discipline_id = :term_discipline_id")?;

    let params = params! {
        "term_discipline_id" => term_discipline_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
