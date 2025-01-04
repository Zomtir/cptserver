use rocket::serde::json::Json;

use crate::common::{Item, ItemCategory, Possession, Stock, WebBool};
use crate::error::Error;
use crate::session::UserSession;

/* ITEMS */

#[rocket::get("/admin/item_list?<category_id>")]
pub fn item_list(session: UserSession, category_id: Option<u32>) -> Result<Json<Vec<Item>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_read {
        return Err(Error::RightInventoryMissing);
    };

    let items = crate::db::inventory::item_list(conn, category_id)?;
    Ok(Json(items))
}

#[rocket::post("/admin/item_create", format = "application/json", data = "<item>")]
pub fn item_create(session: UserSession, item: Json<Item>) -> Result<String, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let id = crate::db::inventory::item_create(conn, &item)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/item_edit?<item_id>", format = "application/json", data = "<item>")]
pub fn item_edit(session: UserSession, item_id: u64, item: Json<Item>) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    crate::db::inventory::item_edit(conn, item_id, &item)?;
    Ok(())
}

#[rocket::head("/admin/item_delete?<item_id>")]
pub fn item_delete(session: UserSession, item_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    crate::db::inventory::item_delete(conn, item_id)?;
    Ok(())
}

/* ITEM CATEGORIES */

#[rocket::get("/admin/itemcat_list")]
pub fn itemcat_list(session: UserSession) -> Result<Json<Vec<ItemCategory>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_read {
        return Err(Error::RightInventoryMissing);
    };

    let itemcats = crate::db::inventory::itemcat_list(conn)?;
    Ok(Json(itemcats))
}

#[rocket::post("/admin/itemcat_create", format = "application/json", data = "<itemcat>")]
pub fn itemcat_create(session: UserSession, itemcat: Json<ItemCategory>) -> Result<String, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let id = crate::db::inventory::itemcat_create(conn, &itemcat)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/itemcat_edit?<category_id>", format = "application/json", data = "<itemcat>")]
pub fn itemcat_edit(session: UserSession, category_id: u64, itemcat: Json<ItemCategory>) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    crate::db::inventory::itemcat_edit(conn, category_id, &itemcat)?;
    Ok(())
}

#[rocket::head("/admin/itemcat_delete?<category_id>")]
pub fn itemcat_delete(session: UserSession, category_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    crate::db::inventory::itemcat_delete(conn, category_id)?;
    Ok(())
}

/* STOCK */

#[rocket::get("/admin/stock_list?<club_id>&<item_id>")]
pub fn stock_list(session: UserSession, club_id: Option<u32>, item_id: Option<u32>) -> Result<Json<Vec<Stock>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_read {
        return Err(Error::RightInventoryMissing);
    };

    let stocks = crate::db::inventory::stock_list(conn, club_id, item_id)?;
    Ok(Json(stocks))
}

#[rocket::post("/admin/stock_create", format = "application/json", data = "<stock>")]
pub fn stock_create(session: UserSession, stock: Json<Stock>) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    if stock.owned > 100 {
        return Err(Error::InventoryStockLimit);
    }

    crate::db::inventory::stock_create(conn, stock.club.id, stock.item.id, &stock.storage, stock.owned)?;

    Ok(())
}

#[rocket::post("/admin/stock_edit?<stock_id>", format = "application/json", data = "<stock>")]
pub fn stock_edit(session: UserSession, stock_id: u64, stock: Json<Stock>) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    if stock.owned > 100 {
        return Err(Error::InventoryStockLimit);
    }

    let db_stock = crate::db::inventory::stock_info(conn, stock_id)?;

    let delta = stock.owned as i64 - db_stock.owned as i64;

    // No change, useless request
    if delta == 0 && db_stock.storage == stock.storage {
        return Ok(());
    }

    // Do not remove loaned items
    if stock.owned < db_stock.loaned {
        return Err(Error::InventoryStockConflict);
    }

    // Check if the client has a different loan information
    if db_stock.loaned != stock.loaned {
        return Err(Error::InventoryStockConflict);
    }

    crate::db::inventory::stock_edit(conn, stock_id, &stock.storage, stock.owned, stock.loaned)?;

    Ok(())
}

#[rocket::head("/admin/stock_delete?<stock_id>")]
pub fn stock_delete(session: UserSession, stock_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let stock = crate::db::inventory::stock_info(conn, stock_id)?;

    // Cannot delete a stock that is incomplete
    if stock.loaned > 0 {
        return Err(Error::InventoryLoanConflict);
    }

    crate::db::inventory::stock_delete(conn, stock_id)?;

    Ok(())
}

#[rocket::head("/admin/item_loan?<stock_id>&<user_id>")]
pub fn item_loan(session: UserSession, stock_id: u64, user_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let stock = crate::db::inventory::stock_info(conn, stock_id)?;

    // No items available to loan
    if stock.owned <= stock.loaned {
        return Err(Error::InventoryStockConflict);
    }

    crate::db::inventory::stock_edit(conn, stock_id, &stock.storage, stock.owned, stock.loaned + 1)?;
    crate::db::inventory::possession_create(
        conn,
        user_id,
        stock.item.id,
        chrono::Utc::now().date_naive(),
        false,
        Some(stock_id),
    )?;

    Ok(())
}

#[rocket::head("/admin/item_return?<possession_id>")]
pub fn item_return(session: UserSession, possession_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let possession = crate::db::inventory::possession_info(conn, possession_id)?;
    let stock = crate::db::inventory::possession_ownership(conn, possession_id)?;

    let stock = match (possession.owned, stock) {
        // Cannot return items which are owned by a user
        (true, None) => return Err(Error::InventoryLoanConflict),
        // Invalid database state, belongs to user but has stock information
        (true, Some(_)) => return Err(Error::DatabaseError),
        // Invalid database state, does not belong to the user but is missing stock information
        (false, None) => return Err(Error::DatabaseError),
        // Does not belong to user, can be returned
        (false, Some(stock)) => stock,
    };

    // Should not happen, but make sure that there are loaned items that can be returned
    if stock.loaned < 1 {
        return Err(Error::DatabaseError);
    }

    crate::db::inventory::stock_edit(conn, stock.id, &stock.storage, stock.owned, stock.loaned - 1)?;
    crate::db::inventory::possession_delete(conn, possession_id)?;

    Ok(())
}

#[rocket::head("/admin/item_handout?<possession_id>")]
pub fn item_handout(session: UserSession, possession_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let possession = crate::db::inventory::possession_info(conn, possession_id)?;
    let stock = crate::db::inventory::possession_ownership(conn, possession_id)?;

    let stock = match (possession.owned, stock) {
        // Cannot hand out items that already belong to a user
        (true, None) => return Err(Error::InventoryLoanConflict),
        // Invalid database state, belongs to user but has stock information
        (true, Some(_)) => return Err(Error::DatabaseError),
        // Invalid database state, does not belong to the user but is missing stock information
        (false, None) => return Err(Error::DatabaseError),
        // Does not belong to user, can be handened out
        (false, Some(stock)) => stock,
    };

    // Should not happen, but make sure that there are loaned items that can be handed out
    if stock.owned < 1 || stock.loaned < 1 {
        return Err(Error::InventoryLoanConflict);
    }

    crate::db::inventory::possession_edit(conn, possession_id, &possession, None)?;
    crate::db::inventory::stock_edit(conn, stock.id, &stock.storage, stock.owned - 1, stock.loaned - 1)?;

    Ok(())
}

#[rocket::head("/admin/item_restock?<possession_id>&<stock_id>")]
pub fn item_restock(session: UserSession, possession_id: u64, stock_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let possession = crate::db::inventory::possession_info(conn, possession_id)?;
    let stock = crate::db::inventory::stock_info(conn, stock_id)?;

    // Cannot restock items on a stock of a different item type
    if possession.item.id != stock.item.id {
        return Err(Error::InventoryStockConflict);
    };

    // Cannot restock items one does not own
    if !possession.owned {
        return Err(Error::InventoryLoanConflict);
    };

    crate::db::inventory::possession_edit(conn, possession_id, &possession, Some(stock_id))?;
    crate::db::inventory::stock_edit(conn, stock_id, &stock.storage, stock.owned + 1, stock.loaned + 1)?;

    Ok(())
}

/* POSSESSIONS */

#[rocket::get("/admin/possession_list?<user_id>&<item_id>&<owned>&<club_id>")]
pub fn possession_list(
    session: UserSession,
    user_id: Option<u64>,
    item_id: Option<u64>,
    owned: Option<WebBool>,
    club_id: Option<u32>,
) -> Result<Json<Vec<Possession>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_read {
        return Err(Error::RightInventoryMissing);
    };

    let possessions =
        crate::db::inventory::possession_list(conn, user_id, item_id, owned.map(|b| b.to_bool()), club_id)?;
    Ok(Json(possessions))
}

#[rocket::head("/admin/possession_create?<user_id>&<item_id>")]
pub fn possession_create(session: UserSession, user_id: u64, item_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    crate::db::inventory::possession_create(conn, user_id, item_id, chrono::Utc::now().date_naive(), true, None)?;
    Ok(())
}

#[rocket::head("/admin/possession_delete?<possession_id>")]
pub fn possession_delete(session: UserSession, possession_id: u64) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let possession = crate::db::inventory::possession_info(conn, possession_id)?;

    // Cannot delete items one does not own
    if !&possession.owned {
        return Err(Error::Default);
    }

    crate::db::inventory::possession_delete(conn, possession_id)?;
    Ok(())
}
