use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::ItemCategory;
use crate::error::ErrorKind;

pub fn itemcat_list(conn: &mut PooledConn) -> Result<Vec<ItemCategory>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT ic.category_id, ic.name
        FROM item_categories ic;",
    )?;

    let params = params::Params::Empty;

    let map = |(category_id, category_name)| ItemCategory {
        id: category_id,
        name: category_name,
    };

    let itemcats = conn.exec_map(&stmt, &params, &map)?;
    Ok(itemcats)
}

pub fn itemcat_create(conn: &mut PooledConn, category: &ItemCategory) -> Result<u32, ErrorKind> {
    let stmt = conn.prep(
        "INSERT INTO item_categories (name)
        SELECT :category_name;",
    )?;

    let params = params! {
        "category_name" => &category.name,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn itemcat_edit(conn: &mut PooledConn, category_id: u64, category: &ItemCategory) -> Result<(), ErrorKind> {
    let stmt = conn.prep(
        "UPDATE item_categories
        SET name = :category_name
        WHERE category_id = :category_id;",
    )?;

    let params = params! {
        "category_id" => &category_id,
        "category_name" => &category.name,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn itemcat_delete(conn: &mut PooledConn, category_id: u64) -> Result<(), ErrorKind> {
    let stmt = conn.prep(
        "DELETE ic
        FROM item_categories ic
        WHERE ic.category_id = :category_id;",
    )?;

    let params = params! {
        "category_id" => category_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
