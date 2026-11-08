use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::{Item, ItemCategory};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub mod club_stock;
pub mod user_equipment;
pub mod user_possession;

#[derive(Deserialize)]
pub struct ItemListQuery {
    pub category_id: Option<u32>,
}

pub async fn item_list(
    State(state): State<AppState>,
    session: UserSession,
    Query(query): Query<ItemListQuery>,
) -> Result<Json<Vec<Item>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)?;

    let items = crate::db::inventory::item_list(conn, query.category_id)?;
    Ok(Json(items))
}

pub async fn item_info(
    State(state): State<AppState>,
    session: UserSession,
    Path(item_id): Path<u32>,
) -> Result<Json<Item>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)?;

    let item = crate::db::inventory::item_info(conn, item_id)?;
    Ok(Json(item))
}

pub async fn item_create(
    State(state): State<AppState>,
    session: UserSession,
    item: Json<Item>,
) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    let item_id = crate::db::inventory::item_create(conn, &item)?;
    Ok(item_id.to_string())
}

pub async fn item_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(item_id): Path<u64>,
    item: Json<Item>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::item_edit(conn, item_id, &item)?;
    Ok(())
}

pub async fn item_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(item_id): Path<u64>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::item_delete(conn, item_id)?;
    Ok(())
}

pub async fn itemcat_list(
    State(state): State<AppState>,
    _session: UserSession,
) -> Result<Json<Vec<ItemCategory>>> {
    let conn = &mut state.db.get_conn()?;
    let item_categories = crate::db::inventory::itemcat_list(conn)?;
    Ok(Json(item_categories))
}

pub async fn itemcat_create(
    State(state): State<AppState>,
    session: UserSession,
    itemcat: Json<ItemCategory>,
) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    let category_id = crate::db::inventory::itemcat_create(conn, &itemcat)?;
    Ok(category_id.to_string())
}

pub async fn itemcat_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(category_id): Path<u64>,
    itemcat: Json<ItemCategory>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::itemcat_edit(conn, category_id, &itemcat)?;
    Ok(())
}

pub async fn itemcat_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(category_id): Path<u64>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::itemcat_delete(conn, category_id)?;
    Ok(())
}
