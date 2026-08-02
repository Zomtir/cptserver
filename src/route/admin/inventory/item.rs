use axum::extract::State;
use axum::Json;

use crate::common::{Item, ItemCategory};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

/* ITEMS */

pub fn item_list(
    State(state): State<AppState>,
    session: UserSession,
    category_id: Option<u32>,
) -> Result<Json<Vec<Item>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)?;

    let items = crate::db::inventory::item_list(conn, category_id)?;
    Ok(Json(items))
}

pub fn item_info(State(state): State<AppState>, session: UserSession, item_id: u32) -> Result<Json<Item>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)?;

    let item = crate::db::inventory::item_info(conn, item_id)?;
    Ok(Json(item))
}

pub fn item_create(State(state): State<AppState>, session: UserSession, item: Json<Item>) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    let id = crate::db::inventory::item_create(conn, &item)?;
    Ok(id.to_string())
}

pub fn item_edit(State(state): State<AppState>, session: UserSession, item_id: u64, item: Json<Item>) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::item_edit(conn, item_id, &item)?;
    Ok(())
}

pub fn item_delete(State(state): State<AppState>, session: UserSession, item_id: u64) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::item_delete(conn, item_id)?;
    Ok(())
}

/* ITEM CATEGORIES */

pub fn itemcat_list(State(state): State<AppState>, session: UserSession) -> Result<Json<Vec<ItemCategory>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)?;

    let itemcats = crate::db::inventory::itemcat_list(conn)?;
    Ok(Json(itemcats))
}

pub fn itemcat_create(
    State(state): State<AppState>,
    session: UserSession,
    itemcat: Json<ItemCategory>,
) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    let id = crate::db::inventory::itemcat_create(conn, &itemcat)?;
    Ok(id.to_string())
}

pub fn itemcat_edit(
    State(state): State<AppState>,
    session: UserSession,
    category_id: u64,
    itemcat: Json<ItemCategory>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::itemcat_edit(conn, category_id, &itemcat)?;
    Ok(())
}

pub fn itemcat_delete(State(state): State<AppState>, session: UserSession, category_id: u64) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::itemcat_delete(conn, category_id)?;
    Ok(())
}
