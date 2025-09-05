use rocket::serde::json::Json;

use crate::common::{Item, ItemCategory};
use crate::error::Result;
use crate::session::UserSession;

/* ITEMS */

#[rocket::get("/admin/item_list?<category_id>")]
pub fn item_list(session: UserSession, category_id: Option<u32>) -> Result<Json<Vec<Item>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)?;

    let items = crate::db::inventory::item_list(conn, category_id)?;
    Ok(Json(items))
}

#[rocket::get("/admin/item_info?<item_id>")]
pub fn item_info(session: UserSession, item_id: u32) -> Result<Json<Item>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)?;

    let item = crate::db::inventory::item_info(conn, item_id)?;
    Ok(Json(item))
}

#[rocket::post("/admin/item_create", format = "application/json", data = "<item>")]
pub fn item_create(session: UserSession, item: Json<Item>) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    let id = crate::db::inventory::item_create(conn, &item)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/item_edit?<item_id>", format = "application/json", data = "<item>")]
pub fn item_edit(session: UserSession, item_id: u64, item: Json<Item>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::item_edit(conn, item_id, &item)?;
    Ok(())
}

#[rocket::head("/admin/item_delete?<item_id>")]
pub fn item_delete(session: UserSession, item_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::item_delete(conn, item_id)?;
    Ok(())
}

/* ITEM CATEGORIES */

#[rocket::get("/admin/itemcat_list")]
pub fn itemcat_list(session: UserSession) -> Result<Json<Vec<ItemCategory>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)?;

    let itemcats = crate::db::inventory::itemcat_list(conn)?;
    Ok(Json(itemcats))
}

#[rocket::post("/admin/itemcat_create", format = "application/json", data = "<itemcat>")]
pub fn itemcat_create(session: UserSession, itemcat: Json<ItemCategory>) -> Result<String> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    let id = crate::db::inventory::itemcat_create(conn, &itemcat)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/itemcat_edit?<category_id>", format = "application/json", data = "<itemcat>")]
pub fn itemcat_edit(session: UserSession, category_id: u64, itemcat: Json<ItemCategory>) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::itemcat_edit(conn, category_id, &itemcat)?;
    Ok(())
}

#[rocket::head("/admin/itemcat_delete?<category_id>")]
pub fn itemcat_delete(session: UserSession, category_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::itemcat_delete(conn, category_id)?;
    Ok(())
}
