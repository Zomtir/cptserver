use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Club, Item, ItemCategory, Possession, Stock, User};
use crate::db::get_pool_conn;
use crate::error::Error;

/* ITEMS */

pub fn item_list(category_id: Option<u32>) -> Result<Vec<Item>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT i.item_id, i.name as item_name, ic.category_id, ic.name as category_name
        FROM items i
        LEFT JOIN item_categories ic ON (i.category_id = ic.category_id)
        WHERE (:category_id IS NULL OR ic.category_id = :category_id);",
    )?;

    let params = params! {
        "category_id" => category_id,
    };

    let map = |(item_id, item_name, category_id, category_name)| Item {
        id: item_id,
        name: item_name,
        category: match (category_id, category_name) {
            (Some(id), Some(name)) => Some(ItemCategory { id, name }),
            _ => None,
        },
    };

    let items = conn.exec_map(&stmt, &params, &map)?;
    Ok(items)
}

pub fn item_create(item: &Item) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
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

pub fn item_edit(item_id: u64, item: &Item) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
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

pub fn item_delete(item_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE i FROM items i WHERE i.item_id = :item_id;")?;

    let params = params! {
        "item_id" => item_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* ITEM CATEGORIES */

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

/* STOCKS */

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

/* POSSESSIONS */

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
