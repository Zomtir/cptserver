use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::Skill;
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn skill_list() -> Result<Vec<Skill>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT skill_id, skill_key, title, min, max
        FROM skills;",
    )?;

    let params = params::Params::Empty;

    let map = |(skill_id, skill_key, title, min, max)| Skill {
        id: skill_id,
        key: skill_key,
        title,
        min,
        max,
    };

    let terms = conn.exec_map(&stmt, &params, &map)?;
    Ok(terms)
}

pub fn skill_create(skill: &Skill) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO skills (skill_key, title, min, max)
        VALUES (:skill_key, :title, :min, :max)",
    )?;

    let params = params! {
        "skill_key" => &skill.key,
        "title" => &skill.title,
        "min" => &skill.min,
        "max" => &skill.max,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn skill_edit(skill_id: u32, skill: &Skill) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE skills SET
            skill_key = :skill_key,
            title = :title,
            min = :min,
            max = :max
        WHERE skill_id = :skill_id",
    )?;

    let params = params! {
        "skill_id" => &skill_id,
        "skill_key" => &skill.key,
        "title" => &skill.title,
        "min" => &skill.min,
        "max" => &skill.max,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn skill_delete(skill_id: u32) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE s FROM skills s WHERE s.skill_id = :skill_id")?;

    let params = params! {
        "skill_id" => skill_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
