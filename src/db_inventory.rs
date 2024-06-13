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
            c.club_id, c.club_key, c.name as club_name, c.description as club_description,
            i.item_id, i.name as item_name, ic.category_id, ic.name as category_name,
            cs.owned, cs.loaned
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
            owned: row.take("owned").unwrap(),
            loaned: row.take("loaned").unwrap(),
        };
        stocks.push(cs);
    }

    Ok(stocks)
}

pub fn stock_info(club_id: u64, item_id: u64) -> Result<Option<Stock>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            c.club_id, c.club_key, c.name as club_name, c.description as club_description,
            i.item_id, i.name as item_name, ic.category_id, ic.name as category_name,
            cs.owned, cs.loaned
        FROM club_stocks cs
        JOIN clubs c ON (cs.club_id = c.club_id)
        JOIN items i ON (cs.item_id = i.item_id)
        LEFT JOIN item_categories ic ON (i.category_id = ic.category_id)
        WHERE cs.club_id = :club_id
        AND cs.item_id = :item_id;",
    )?;

    let params = params! {
        "club_id" => club_id,
        "item_id" => item_id,
    };

    let mut row: mysql::Row = match conn.exec_first(&stmt, &params)? {
        None => return Ok(None),
        Some(row) => row,
    };

    let stock = Stock {
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
        owned: row.take("owned").unwrap(),
        loaned: row.take("loaned").unwrap(),
    };

    Ok(Some(stock))
}

pub fn stock_create(club_id: u64, item_id: u64, owned: u32) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO club_stocks (club_id, item_id, owned, loaned)
        SELECT :club_id, :item_id, :owned, :loaned;",
    )?;

    let params = params! {
        "club_id" => &club_id,
        "item_id" => &item_id,
        "owned" => &owned,
        "loaned" => 0,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(())
}

pub fn stock_edit(club_id: u64, item_id: u64, owned: u32, loaned: u32) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE club_stocks
        SET
            club_id  = :club_id,
            item_id = :item_id,
            owned = :owned,
            loaned = :loaned
        WHERE club_id = :club_id
        AND item_id = :item_id;",
    )?;

    let params = params! {
        "club_id" => &club_id,
        "item_id" => &item_id,
        "owned" => &owned,
        "loaned" => &loaned,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn stock_delete(club_id: u64, item_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE cs FROM club_stocks cs
        WHERE cs.club_id = :club_id
        AND cs.item_id = :item_id;",
    )?;

    let params = params! {
        "club_id" => club_id,
        "item_id" => item_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* POSSESSIONS */

pub fn possession_list(
    user_id: Option<u64>,
    item_id: Option<u64>,
    owned: Option<bool>,
    club_id: Option<u32>,
) -> Result<Vec<Possession>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT up.possession_id, up.owned,
            u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
            i.item_id, i.name as item_name, ic.category_id, ic.name as category_name,
            c.club_id, c.club_key, c.name as club_name, c.description as club_description,
            up.transfer_date
        FROM user_possessions up
        JOIN users u ON (up.user_id = u.user_id)
        JOIN items i ON (up.item_id = i.item_id)
        LEFT JOIN item_categories ic ON (i.category_id = ic.category_id)
        LEFT JOIN clubs c ON (up.club_id = c.club_id)
        WHERE (:user_id IS NULL OR up.user_id = :user_id)
        AND (:item_id IS NULL OR up.item_id = :item_id)
        AND (:owned IS NULL OR up.owned = :owned)
        AND (:club_id IS NULL OR up.club_id = :club_id);",
    )?;

    let params = params! {
        "user_id" => user_id,
        "item_id" => item_id,
        "owned" => owned,
        "club_id" => club_id,
    };

    let rows: Vec<mysql::Row> = conn.exec(&stmt, &params)?;

    let mut possessions: Vec<Possession> = Vec::new();

    for mut row in rows {
        let up = Possession {
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
            owned: row.take("owned").unwrap(),
            club: row.take::<Option<u64>, &str>("club_id").unwrap().map(|id| Club {
                id,
                key: row.take("club_key").unwrap(),
                name: row.take("club_name").unwrap(),
                description: row.take("club_description").unwrap(),
            }),
            transfer_date: row.take("transfer_date").unwrap(),
        };
        possessions.push(up);
    }

    Ok(possessions)
}

pub fn possession_info(possession_id: u64) -> Result<Possession, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT up.possession_id, up.owned,
            u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
            i.item_id, i.name as item_name, ic.category_id, ic.name as category_name,
            c.club_id, c.club_key, c.name as club_name, c.description as club_description,
            up.transfer_date
        FROM user_possessions up
        JOIN users u ON (up.user_id = u.user_id)
        JOIN items i ON (up.item_id = i.item_id)
        LEFT JOIN item_categories ic ON (i.category_id = ic.category_id)
        LEFT JOIN clubs c ON (up.club_id = c.club_id)
        WHERE (up.possession_id = :possession_id);",
    )?;

    let params = params! {
        "possession_id" => possession_id,
    };

    let mut row: mysql::Row = match conn.exec_first(&stmt, &params)? {
        None => return Err(Error::InventoryPossessionMissing),
        Some(row) => row,
    };

    let possession = Possession {
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
        owned: row.take("owned").unwrap(),
        club: row.take::<Option<u64>, &str>("club_id").unwrap().map(|id| Club {
            id,
            key: row.take("club_key").unwrap(),
            name: row.take("club_name").unwrap(),
            description: row.take("club_description").unwrap(),
        }),

        transfer_date: row.take("transfer_date").unwrap(),
    };

    Ok(possession)
}

pub fn possession_create(
    user_id: u64,
    item_id: u64,
    owned: bool,
    club_id: Option<u64>,
    transfer_date: Option<chrono::NaiveDate>,
) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO user_possessions (user_id, item_id, owned, club_id, transfer_date)
        SELECT :user_id, :item_id, :owned, :club_id, :transfer_date;",
    )?;

    let params = params! {
        "user_id" => &user_id,
        "item_id" => &item_id,
        "owned" => &owned,
        "club_id" => &club_id,
        "transfer_date" => &transfer_date,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn possession_edit(possession_id: u64, possession: &Possession) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE user_possessions
        SET
            user_id  = :user_id,
            item_id = :item_id,
            owned = :owned,
            club_id = :club_id,
            transfer_date = :transfer_date
        WHERE possession_id = :possession_id;",
    )?;

    let params = params! {
        "possession_id" => &possession_id,
        "user_id" => &possession.user.id,
        "item_id" => &possession.item.id,
        "owned" => &possession.owned,
        "club_id" => &possession.club.as_ref().map(|club| club.id),
        "transfer_date" => &possession.transfer_date,
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
