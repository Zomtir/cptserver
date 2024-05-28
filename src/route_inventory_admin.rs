use rocket::serde::json::Json;

use crate::common::{Item, ItemCategory, Possession, Stock, WebBool};
use crate::error::Error;
use crate::session::UserSession;

/* ITEMS */

#[rocket::get("/admin/item_list?<category_id>")]
pub fn item_list(session: UserSession, category_id: Option<u32>) -> Result<Json<Vec<Item>>, Error> {
    if !session.right.right_inventory_read {
        return Err(Error::RightInventoryMissing);
    };

    let items = crate::db_inventory::item_list(category_id)?;
    Ok(Json(items))
}

#[rocket::post("/admin/item_create", format = "application/json", data = "<item>")]
pub fn item_create(session: UserSession, item: Json<Item>) -> Result<String, Error> {
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let id = crate::db_inventory::item_create(&item)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/item_edit?<item_id>", format = "application/json", data = "<item>")]
pub fn item_edit(session: UserSession, item_id: u64, item: Json<Item>) -> Result<(), Error> {
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    crate::db_inventory::item_edit(item_id, &item)?;
    Ok(())
}

#[rocket::head("/admin/item_delete?<item_id>")]
pub fn item_delete(session: UserSession, item_id: u64) -> Result<(), Error> {
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    crate::db_inventory::item_delete(item_id)?;
    Ok(())
}

/* ITEM CATEGORIES */

#[rocket::get("/admin/itemcat_list")]
pub fn itemcat_list(session: UserSession) -> Result<Json<Vec<ItemCategory>>, Error> {
    if !session.right.right_inventory_read {
        return Err(Error::RightInventoryMissing);
    };

    let itemcats = crate::db_inventory::itemcat_list()?;
    Ok(Json(itemcats))
}

#[rocket::post("/admin/itemcat_create", format = "application/json", data = "<itemcat>")]
pub fn itemcat_create(session: UserSession, itemcat: Json<ItemCategory>) -> Result<String, Error> {
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let id = crate::db_inventory::itemcat_create(&itemcat)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/itemcat_edit?<category_id>", format = "application/json", data = "<itemcat>")]
pub fn itemcat_edit(session: UserSession, category_id: u64, itemcat: Json<ItemCategory>) -> Result<(), Error> {
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    crate::db_inventory::itemcat_edit(category_id, &itemcat)?;
    Ok(())
}

#[rocket::head("/admin/itemcat_delete?<category_id>")]
pub fn itemcat_delete(session: UserSession, category_id: u64) -> Result<(), Error> {
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    crate::db_inventory::itemcat_delete(category_id)?;
    Ok(())
}

/* STOCK */

#[rocket::get("/admin/stock_list?<club_id>&<item_id>")]
pub fn stock_list(session: UserSession, club_id: Option<u32>, item_id: Option<u32>) -> Result<Json<Vec<Stock>>, Error> {
    if !session.right.right_inventory_read {
        return Err(Error::RightInventoryMissing);
    };

    let stocks = crate::db_inventory::stock_list(club_id, item_id)?;
    Ok(Json(stocks))
}

#[rocket::post(
    "/admin/stock_edit?<club_id>&<item_id>",
    format = "application/json",
    data = "<stock>"
)]
pub fn stock_edit(session: UserSession, club_id: u64, item_id: u64, stock: Json<Stock>) -> Result<(), Error> {
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    if stock.owned > 100 {
        return Err(Error::InventoryStockLimit);
    }

    let (db_owned, db_loaned) = match crate::db_inventory::stock_info(club_id, item_id)? {
        None => (0, 0),
        Some(db_stock) => (db_stock.owned, db_stock.loaned),
    };

    let overhead = db_owned as i64 - db_loaned as i64;
    let delta = stock.owned as i64 - db_owned as i64;

    // Having more loaned items than stocked items is usually bad
    if overhead < 0 {
        return Err(Error::InventoryStockInvalid);
    }

    // No change, useless request
    if delta == 0 {
        return Ok(());
    }

    // Do not remove loaned items
    if overhead + delta < 0 {
        return Err(Error::InventoryStockConflict);
    }

    // Check if the client has a different loan information
    if db_loaned != stock.loaned {
        return Err(Error::InventoryStockConflict);
    }

    // So far no conflicts, apply the change
    match (db_owned, stock.owned) {
        (0, 0) => (), // This should never be reached due to the safeguard
        (0, _) => crate::db_inventory::stock_create(club_id, item_id, stock.owned)?,
        (_, 0) => crate::db_inventory::stock_delete(club_id, item_id)?,
        (_, _) => crate::db_inventory::stock_edit(club_id, item_id, stock.owned, stock.loaned)?,
    }

    Ok(())
}

#[rocket::head("/admin/item_loan?<club_id>&<item_id>&<user_id>")]
pub fn item_loan(session: UserSession, club_id: u64, item_id: u64, user_id: u64) -> Result<(), Error> {
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let (db_owned, db_loaned) = match crate::db_inventory::stock_info(club_id, item_id)? {
        None => (0, 0),
        Some(db_stock) => (db_stock.owned, db_stock.loaned),
    };

    let overhead = db_owned as i64 - db_loaned as i64;

    // No items available to loan
    if overhead < 1 {
        return Err(Error::InventoryStockConflict);
    }

    crate::db_inventory::stock_edit(club_id, item_id, db_owned, db_loaned + 1)?;
    crate::db_inventory::possession_create(
        user_id,
        item_id,
        false,
        Some(club_id),
        Some(chrono::Utc::now().date_naive()),
    )?;

    Ok(())
}

#[rocket::head("/admin/item_return?<possession_id>")]
pub fn item_return(session: UserSession, possession_id: u64) -> Result<(), Error> {
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let possession = crate::db_inventory::possession_info(possession_id)?;

    let club_id = match (possession.owned, possession.club, possession.transfer_date) {
        // Cannot return items which are owned by a user
        (true, _, _) => return Err(Error::Default),
        (false, Some(club), Some(_)) => club.id,
        // Either club or transfer date is null, therefor it is an internal error
        _ => return Err(Error::Default),
    };
    let item_id = possession.item.id;

    let (db_owned, db_loaned) = match crate::db_inventory::stock_info(club_id, item_id)? {
        None => (0, 0),
        Some(db_stock) => (db_stock.owned, db_stock.loaned),
    };

    // Should not happen, but make sure that there are loaned items that can be returned
    if db_loaned < 1 {
        return Err(Error::Default);
    }

    crate::db_inventory::stock_edit(club_id, item_id, db_owned, db_loaned - 1)?;
    crate::db_inventory::possession_delete(possession_id)?;

    Ok(())
}

#[rocket::head("/admin/item_handout?<possession_id>")]
pub fn item_handout(session: UserSession, possession_id: u64) -> Result<(), Error> {
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let mut possession = crate::db_inventory::possession_info(possession_id)?;

    let club_id = match (&possession.owned, &possession.club, &possession.transfer_date) {
        // Cannot hand out items twice
        (true, _, _) => return Err(Error::Default),
        (false, Some(club), Some(_)) => club.id,
        // Either club or transfer date is null, therefor it is an internal error
        _ => return Err(Error::Default),
    };
    let item_id = possession.item.id;

    let (db_owned, db_loaned) = match crate::db_inventory::stock_info(club_id, item_id)? {
        None => (0, 0),
        Some(db_stock) => (db_stock.owned, db_stock.loaned),
    };

    // Should not happen, but make sure that there are loaned items that can be handed out
    if db_owned < 1 || db_loaned < 1 {
        return Err(Error::Default);
    }

    possession.owned = true;
    crate::db_inventory::possession_edit(possession_id, &possession)?;
    crate::db_inventory::stock_edit(club_id, item_id, db_owned - 1, db_loaned - 1)?;

    Ok(())
}

#[rocket::head("/admin/item_handback?<possession_id>")]
pub fn item_handback(session: UserSession, possession_id: u64) -> Result<(), Error> {
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let mut possession = crate::db_inventory::possession_info(possession_id)?;

    let club_id = match (&possession.owned, &possession.club, &possession.transfer_date) {
        // Cannot hand back items one does not own
        (false, _, _) => return Err(Error::Default),
        (true, Some(club), Some(_)) => club.id,
        // Either club or transfer date is null, therefor it is an internal error
        _ => return Err(Error::Default),
    };
    let item_id = possession.item.id;

    let (db_owned, db_loaned) = match crate::db_inventory::stock_info(club_id, item_id)? {
        None => (0, 0),
        Some(db_stock) => (db_stock.owned, db_stock.loaned),
    };

    possession.owned = false;
    crate::db_inventory::possession_edit(possession_id, &possession)?;
    crate::db_inventory::stock_edit(club_id, item_id, db_owned + 1, db_loaned + 1)?;

    Ok(())
}

/* POSSESSIONS */

#[rocket::get("/admin/possession_list?<user_id>&<owned>&<club_id>")]
pub fn possession_list(
    session: UserSession,
    user_id: Option<u32>,
    owned: Option<WebBool>,
    club_id: Option<u32>,
) -> Result<Json<Vec<Possession>>, Error> {
    if !session.right.right_inventory_read {
        return Err(Error::RightInventoryMissing);
    };

    let possessions = crate::db_inventory::possession_list(user_id, owned.map(|b| b.to_bool()), club_id)?;
    Ok(Json(possessions))
}

#[rocket::head("/admin/possession_create?<user_id>&<item_id>")]
pub fn possession_create(session: UserSession, user_id: u64, item_id: u64) -> Result<(), Error> {
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    crate::db_inventory::possession_create(user_id, item_id, true, None, None)?;
    Ok(())
}

#[rocket::head("/admin/possession_delete?<possession_id>")]
pub fn possession_delete(session: UserSession, possession_id: u64) -> Result<(), Error> {
    if !session.right.right_inventory_write {
        return Err(Error::RightInventoryMissing);
    };

    let possession = crate::db_inventory::possession_info(possession_id)?;

    // Cannot delete items one does not own
    if !&possession.owned {
        return Err(Error::Default);
    }

    crate::db_inventory::possession_delete(possession_id)?;
    Ok(())
}
