use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Club, Item, ItemCategory, Possession, Stock, User};
use crate::db::get_pool_conn;
use crate::error::Error;

fn sql_possession(mut row: mysql::Row) -> Possession {
    Possession {
        id: row.take("possession_id").unwrap(),
        user: User::from_info(
            row.take("user_id").unwrap(),
            row.take("user_key").unwrap(),
            row.take("firstname").unwrap(),
            row.take("lastname").unwrap(),
            row.take("nickname").unwrap(),
        ),
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
        acquisition_date: row.take("acquisition_date").unwrap(),
        owned: row.take("owned").unwrap(),
    }
}

pub fn possession_list(
    user_id: Option<u64>,
    item_id: Option<u64>,
    owned: Option<bool>,
    club_id: Option<u32>,
) -> Result<Vec<Possession>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT up.possession_id,
            u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
            i.item_id, i.name as item_name, ic.category_id, ic.name as category_name,
            up.acquisition_date, up.owned
        FROM user_possessions up
        JOIN users u ON (up.user_id = u.user_id)
        JOIN items i ON (up.item_id = i.item_id)
        LEFT JOIN item_categories ic ON (i.category_id = ic.category_id)
        LEFT JOIN club_stocks cs ON (up.stock_id = cs.stock_id)
        WHERE (:user_id IS NULL OR up.user_id = :user_id)
        AND (:item_id IS NULL OR up.item_id = :item_id)
        AND (:owned IS NULL OR up.owned = :owned)
        AND (:club_id IS NULL OR cs.club_id = :club_id);",
    )?;

    let params = params! {
        "user_id" => user_id,
        "item_id" => item_id,
        "owned" => owned,
        "club_id" => club_id,
    };

    let rows: Vec<mysql::Row> = conn.exec(&stmt, &params)?;

    let mut possessions: Vec<Possession> = Vec::new();

    for row in rows {
        possessions.push(sql_possession(row));
    }

    Ok(possessions)
}

pub fn possession_info(possession_id: u64) -> Result<Possession, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT up.possession_id,
            u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
            i.item_id, i.name as item_name, ic.category_id, ic.name as category_name,
            up.acquisition_date, up.owned
        FROM user_possessions up
        JOIN users u ON (up.user_id = u.user_id)
        JOIN items i ON (up.item_id = i.item_id)
        LEFT JOIN item_categories ic ON (i.category_id = ic.category_id)
        WHERE (up.possession_id = :possession_id);",
    )?;

    let params = params! {
        "possession_id" => possession_id,
    };

    let row: mysql::Row = match conn.exec_first(&stmt, &params)? {
        None => return Err(Error::InventoryPossessionMissing),
        Some(row) => row,
    };

    Ok(sql_possession(row))
}

pub fn possession_ownership(possession_id: u64) -> Result<Option<Stock>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT cs.stock_id,
            c.club_id, c.club_key, c.name as club_name, c.description as club_description,
            i.item_id, i.name as item_name, ic.category_id, ic.name as category_name,
            cs.storage, cs.owned, cs.loaned
        FROM user_possessions up
        JOIN users u ON (up.user_id = u.user_id)
        JOIN items i ON (up.item_id = i.item_id)
        LEFT JOIN item_categories ic ON (i.category_id = ic.category_id)
        LEFT JOIN club_stocks cs ON (up.stock_id = cs.stock_id)
        LEFT JOIN clubs c ON (c.club_id = cs.club_id)
        WHERE (up.possession_id = :possession_id);",
    )?;

    let params = params! {
        "possession_id" => possession_id,
    };

    let mut row: mysql::Row = match conn.exec_first(&stmt, &params)? {
        None => return Ok(None),
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

    Ok(Some(stock))
}

pub fn possession_create(
    user_id: u64,
    item_id: u64,
    acquisition_date: chrono::NaiveDate,
    owned: bool,
    stock_id: Option<u64>,
) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO user_possessions (user_id, item_id, acquisition_date, owned, stock_id)
        SELECT :user_id, :item_id, :acquisition_date, :owned, :stock_id;",
    )?;

    let params = params! {
        "user_id" => &user_id,
        "item_id" => &item_id,
        "acquisition_date" => &acquisition_date,
        "owned" => &owned,
        "stock_id" => &stock_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn possession_edit(possession_id: u64, possession: &Possession, stock_id: Option<u64>) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE user_possessions
        SET
            user_id  = :user_id,
            item_id = :item_id,
            acquisition_date = :acquisition_date,
            owned = :owned,
            stock_id = :stock_id
        WHERE possession_id = :possession_id;",
    )?;

    let params = params! {
        "possession_id" => &possession_id,
        "user_id" => &possession.user.id,
        "item_id" => &possession.item.id,
        "acquisition_date" => &possession.acquisition_date,
        "owned" => stock_id.is_none(),
        "stock_id" => stock_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn possession_delete(possession_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE up
        FROM user_possessions up
        WHERE up.possession_id = :possession_id;",
    )?;

    let params = params! {
        "possession_id" => possession_id
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
