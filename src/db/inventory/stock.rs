use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Club, Item, ItemCategory, Stock};
use crate::db::get_pool_conn;
use crate::error::Error;

pub fn stock_list(club_id: Option<u32>, item_id: Option<u32>) -> Result<Vec<Stock>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            cs.stock_id,
            c.club_id, c.club_key, c.name as club_name, c.description as club_description,
            i.item_id, i.name as item_name, ic.category_id, ic.name as category_name,
            cs.storage, cs.owned, cs.loaned
        FROM club_stocks cs
        JOIN clubs c ON (cs.club_id = c.club_id)
        JOIN items i ON (cs.item_id = i.item_id)
        LEFT JOIN item_categories ic ON (i.category_id = ic.category_id)
        WHERE (:club_id IS NULL OR cs.club_id = :club_id)
        AND (:item_id IS NULL OR cs.item_id = :item_id);",
    )?;

    let params = params! {
        "club_id" => club_id,
        "item_id" => item_id,
    };

    let rows: Vec<mysql::Row> = conn.exec(&stmt, &params)?;

    let mut stocks: Vec<Stock> = Vec::new();

    for mut row in rows {
        let cs = Stock {
            id: row.take("stock_id").unwrap(),
            club: Club {
                id: row.take("club_id").unwrap(),
                key: row.take("club_key").unwrap(),
                name: row.take("club_name").unwrap(),
                description: row.take("club_description").unwrap(),
            },
            item: Item {
                id: row.take("item_id").unwrap(),
                name: row.take("item_name").unwrap(),
                category: row
                    .take::<Option<u64>, &str>("category_id")
                    .unwrap()
                    .map(|id| ItemCategory {
                        id,
                        name: row.take("category_name").unwrap(),
                    }),
            },
            storage: row.take("storage").unwrap(),
            owned: row.take("owned").unwrap(),
            loaned: row.take("loaned").unwrap(),
        };
        stocks.push(cs);
    }

    Ok(stocks)
}

pub fn stock_info(stock_id: u64) -> Result<Stock, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            cs.stock_id,
            c.club_id, c.club_key, c.name as club_name, c.description as club_description,
            i.item_id, i.name as item_name, ic.category_id, ic.name as category_name,
            cs.storage, cs.owned, cs.loaned
        FROM club_stocks cs
        JOIN clubs c ON (cs.club_id = c.club_id)
        JOIN items i ON (cs.item_id = i.item_id)
        LEFT JOIN item_categories ic ON (i.category_id = ic.category_id)
        WHERE cs.stock_id = :stock_id;",
    )?;

    let params = params! {
        "stock_id" => stock_id,
    };

    let mut row: mysql::Row = match conn.exec_first(&stmt, &params)? {
        None => return Err(Error::InventoryStockMissing),
        Some(row) => row,
    };

    let stock = Stock {
        id: row.take("stock_id").unwrap(),
        club: Club {
            id: row.take("club_id").unwrap(),
            key: row.take("club_key").unwrap(),
            name: row.take("club_name").unwrap(),
            description: row.take("club_description").unwrap(),
        },
        item: Item {
            id: row.take("item_id").unwrap(),
            name: row.take("item_name").unwrap(),
            category: row
                .take::<Option<u64>, &str>("category_id")
                .unwrap()
                .map(|id| ItemCategory {
                    id,
                    name: row.take("category_name").unwrap(),
                }),
        },
        storage: row.take("storage").unwrap(),
        owned: row.take("owned").unwrap(),
        loaned: row.take("loaned").unwrap(),
    };

    Ok(stock)
}

pub fn stock_create(club_id: u64, item_id: u64, storage: &String, owned: u32) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO club_stocks (club_id, item_id, storage, owned, loaned)
        SELECT :club_id, :item_id, :storage, :owned, :loaned;",
    )?;

    let params = params! {
        "club_id" => &club_id,
        "item_id" => &item_id,
        "storage" => storage,
        "owned" => &owned,
        "loaned" => 0,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(())
}

pub fn stock_edit(stock_id: u64, storage: &String, owned: u32, loaned: u32) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE club_stocks
        SET
            storage = :storage,
            owned = :owned,
            loaned = :loaned
        WHERE stock_id = :stock_id;",
    )?;

    let params = params! {
        "stock_id" => &stock_id,
        "storage" => &storage,
        "owned" => &owned,
        "loaned" => &loaned,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn stock_delete(stock_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE cs FROM club_stocks cs
        WHERE cs.stock_id = :stock_id;",
    )?;

    let params = params! {
        "stock_id" => stock_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
