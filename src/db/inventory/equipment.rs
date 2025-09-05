use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Equipment, Item, Skill, User};
use crate::error::{Error, ErrorKind, Result};

pub fn user_equipment_list(
    conn: &mut PooledConn,
    user_id: Option<u64>,
    skill_id: Option<u16>,
    item_id: Option<u64>,
) -> Result<Vec<Equipment>> {
    let stmt = conn.prep(
        "SELECT
            ue.equipment_id,
            u.user_id, u.user_key, u.firstname AS user_firstname, u.lastname AS user_lastname, u.nickname AS user_nickname,
            s.skill_id, s.skill_key, s.name as skill_name,
            i.item_id, i.name as item_name, ic.category_id, ic.name as category_name,
            ue.count
        FROM user_equipment ue
        JOIN users u ON (ue.user_id = u.user_id)
        JOIN skills s ON (ue.skill_id = s.skill_id)
        JOIN items i ON (ue.item_id = i.item_id)
        LEFT JOIN item_categories ic ON (i.category_id = ic.category_id)
        WHERE (:user_id IS NULL OR ue.user_id = :user_id)
        AND (:skill_id IS NULL OR ue.skill_id = :skill_id)
        AND (:item_id IS NULL OR ue.item_id = :item_id);",
    )?;

    let params = params! {
        "user_id" => user_id,
        "skill_id" => skill_id,
        "item_id" => item_id,
    };

    let rows: Vec<mysql::Row> = conn.exec(&stmt, &params)?;

    let mut equipments = Vec::new();

    for mut row in rows {
        let equipment = Equipment {
            id: row.take("equipment_id").unwrap(),
            user: User::from_info(
                row.take("user_id").unwrap(),
                row.take("user_key").unwrap(),
                row.take("user_firstname").unwrap(),
                row.take("user_lastname").unwrap(),
                row.take("user_nickname").unwrap(),
            ),
            skill: Skill::from_row(
                row.take("skill_id"),
                row.take("skill_key"),
                row.take("skill_name"),
                None,
                None,
            )
            .unwrap(),
            item: Item::from_row(
                row.take("item_id").unwrap(),
                row.take("item_name").unwrap(),
                row.take("category_id").unwrap(),
                row.take("category_name").unwrap(),
            )
            .unwrap(),
            count: row.take("count").unwrap(),
        };

        equipments.push(equipment);
    }

    Ok(equipments)
}

pub fn user_equipment_info(conn: &mut PooledConn, equipment_id: u64) -> Result<Equipment> {
    let stmt = conn.prep(
        "SELECT
            ue.equipment_id,
            u.user_id, u.user_key, u.firstname AS user_firstname, u.lastname AS user_lastname, u.nickname AS user_nickname,
            s.skill_id, s.skill_key, s.name as skill_name,
            i.item_id, i.name as item_name, ic.category_id, ic.name as category_name,
            ue.count
        FROM user_equipment ue
        JOIN users u ON (ue.user_id = u.user_id)
        JOIN skills s ON (ue.skill_id = s.skill_id)
        JOIN items i ON (cs.item_id = i.item_id)
        LEFT JOIN item_categories ic ON (i.category_id = ic.category_id)
        WHERE ue.equipment_id = :equipment_id;",
    )?;

    let params = params! {
        "equipment_id" => equipment_id,
    };

    let mut row: mysql::Row = match conn.exec_first(&stmt, &params)? {
        None => return Err(Error::new(ErrorKind::Missing, "Equipment is missing")),
        Some(row) => row,
    };

    let equipment = Equipment {
        id: row.take("equipment_id").unwrap(),
        user: User::from_info(
            row.take("user_id").unwrap(),
            row.take("user_key").unwrap(),
            row.take("user_firstname").unwrap(),
            row.take("user_lastname").unwrap(),
            row.take("user_nickname").unwrap(),
        ),
        skill: Skill::from_row(
            row.take("skill_id").unwrap(),
            row.take("skill_key").unwrap(),
            row.take("skill_name").unwrap(),
            None,
            None,
        )
        .unwrap(),
        item: Item::from_row(
            row.take("item_id").unwrap(),
            row.take("item_name").unwrap(),
            row.take("category_id").unwrap(),
            row.take("category_name").unwrap(),
        )
        .unwrap(),
        count: row.take("count").unwrap(),
    };

    Ok(equipment)
}

pub fn user_equipment_create(
    conn: &mut PooledConn,
    user_id: u64,
    skill_id: u16,
    item_id: u64,
    count: u32,
) -> Result<()> {
    let stmt = conn.prep(
        "INSERT INTO user_equipment (
            user_id,
            skill_id,
            item_id,
            count
        )
        VALUES (
            :user_id,
            :skill_id,
            :item_id,
            :count
        );",
    )?;

    let params = params! {
        "user_id" => user_id,
        "skill_id" => skill_id,
        "item_id" => item_id,
        "count" => count,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(())
}

pub fn user_equipment_edit(conn: &mut PooledConn, equipment_id: u64, count: u32) -> Result<()> {
    let stmt = conn.prep(
        "UPDATE user_equipment
        SET
            count = :count
        WHERE equipment_id = :equipment_id;",
    )?;

    let params = params! {
        "equipment_id" => equipment_id,
        "count" => count,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(())
}

pub fn user_equipment_delete(conn: &mut PooledConn, equipment_id: u64) -> Result<()> {
    let stmt = conn.prep(
        "DELETE FROM user_equipment
        WHERE equipment_id = :equipment_id;",
    )?;

    let params = params! {
        "equipment_id" => equipment_id,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(())
}
