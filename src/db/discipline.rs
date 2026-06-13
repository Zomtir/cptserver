use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::Discipline;
use crate::error::Result;

pub fn discipline_list(conn: &mut PooledConn) -> Result<Vec<Discipline>> {
    let stmt = conn.prep(
        "SELECT discipline_id, name
        FROM disciplines;",
    )?;

    let params = params::Params::Empty;

    let map = |(discipline_id, name)| Discipline {
        id: discipline_id,
        name,
    };

    let terms = conn.exec_map(&stmt, &params, &map)?;
    Ok(terms)
}

pub fn discipline_create(conn: &mut PooledConn, discipline: &Discipline) -> Result<u32> {
    let stmt = conn.prep(
        "INSERT INTO disciplines (name)
        VALUES (:name)",
    )?;

    let params = params! {
        "name" => &discipline.name,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn discipline_edit(conn: &mut PooledConn, discipline_id: u32, discipline: &Discipline) -> Result<()> {
    let stmt = conn.prep(
        "UPDATE disciplines SET
            name = :name
        WHERE discipline_id = :discipline_id",
    )?;

    let params = params! {
        "discipline_id" => &discipline_id,
        "name" => &discipline.name,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn discipline_delete(conn: &mut PooledConn, discipline_id: u32) -> Result<()> {
    let stmt = conn.prep("DELETE s FROM disciplines s WHERE s.discipline_id = :discipline_id")?;

    let params = params! {
        "discipline_id" => discipline_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
