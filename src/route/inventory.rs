use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::{Item, ItemCategory, Possession, WebBool};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

#[derive(Deserialize)]
pub struct ItemListQuery {
    pub category_id: Option<u32>,
}

#[derive(Deserialize)]
pub struct PossessionListQuery {
    pub user_id: Option<u64>,
    pub item_id: Option<u64>,
    pub owned: Option<WebBool>,
    pub club_id: Option<u32>,
}

#[derive(Deserialize)]
pub struct PossessionCreateQuery {
    pub user_id: u64,
    pub item_id: u64,
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

pub async fn possession_list(
    State(state): State<AppState>,
    session: UserSession,
    Query(query): Query<PossessionListQuery>,
) -> Result<Json<Vec<Possession>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)
        .or(crate::permission::is_self(&session, query.user_id))?;

    let possessions = crate::db::inventory::possession_list(
        conn,
        query.user_id,
        query.item_id,
        query.owned.map(|value| value.to_bool()),
        query.club_id,
    )?;
    Ok(Json(possessions))
}

pub async fn possession_create(
    State(state): State<AppState>,
    session: UserSession,
    Query(query): Query<PossessionCreateQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::possession_create(
        conn,
        query.user_id,
        query.item_id,
        chrono::Utc::now().date_naive(),
        true,
        None,
    )?;
    Ok(())
}

pub async fn possession_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(possession_id): Path<u64>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    let possession = crate::db::inventory::possession_info(conn, possession_id)?;
    if !possession.owned {
        return Err(crate::error::Error::new(
            crate::error::ErrorKind::Permission,
            "Cannot delete items that are not owned",
        ));
    }

    crate::db::inventory::possession_delete(conn, possession_id)?;
    Ok(())
}
