use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Item, ItemCategory};
use crate::error::ErrorKind;

fn row_map((item_id, item_name, category_id, category_name): (u64, String, Option<u64>, Option<String>)) -> Item {
    Item {
        id: item_id,
        name: item_name,
        category: match (category_id, category_name) {
            (Some(id), Some(name)) => Some(ItemCategory { id, name }),
            _ => None,
        },
    }
}

pub fn item_list(conn: &mut PooledConn, category_id: Option<u32>) -> Result<Vec<Item>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT i.item_id, i.name as item_name, ic.category_id, ic.name as category_name
        FROM items i
        LEFT JOIN item_categories ic ON (i.category_id = ic.category_id)
        WHERE (:category_id IS NULL OR ic.category_id = :category_id);",
    )?;

    let params = params! {
        "category_id" => category_id,
    };

    let items = conn.exec_map(&stmt, &params, &row_map)?;
    Ok(items)
}

pub fn item_info(conn: &mut PooledConn, item_id: u32) -> Result<Item, ErrorKind> {
    let stmt = conn.prep(
        "SELECT i.item_id, i.name as item_name, ic.category_id, ic.name as category_name
        FROM items i
        LEFT JOIN item_categories ic ON (i.category_id = ic.category_id)
        WHERE :item_id = i.item_id;",
    )?;

    let params = params! {
        "item_id" => item_id,
    };

    let row = conn.exec_first(&stmt, &params)?;
    row.map(row_map).ok_or(ErrorKind::Missing)
}

pub fn item_create(conn: &mut PooledConn, item: &Item) -> Result<u32, ErrorKind> {
    let stmt = conn.prep(
        "INSERT INTO items (name, category_id)
        SELECT :item_name, :category_id;",
    )?;

    let params = params! {
        "item_name" => &item.name,
        "category_id" => &item.category.as_ref().map(|category| category.id),
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn item_edit(conn: &mut PooledConn, item_id: u64, item: &Item) -> Result<(), ErrorKind> {
    let stmt = conn.prep(
        "UPDATE items
        SET name = :item_name, category_id = :category_id
        WHERE item_id = :item_id;",
    )?;

    let params = params! {
        "item_id" => &item_id,
        "item_name" => &item.name,
        "category_id" => &item.category.as_ref().map(|category| category.id),
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn item_delete(conn: &mut PooledConn, item_id: u64) -> Result<(), ErrorKind> {
    let stmt = conn.prep("DELETE i FROM items i WHERE i.item_id = :item_id;")?;

    let params = params! {
        "item_id" => item_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
