use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::ItemCategory;
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn itemcat_list() -> Result<Vec<ItemCategory>, Error> {
    let mut conn: PooledConn = get_pool_conn();
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

pub fn itemcat_create(category: &ItemCategory) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
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

pub fn itemcat_edit(category_id: u64, category: &ItemCategory) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
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

pub fn itemcat_delete(category_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
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
