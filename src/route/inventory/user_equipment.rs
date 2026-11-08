use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::Equipment;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

#[derive(Deserialize)]
pub struct UserEquipmentListQuery {
    pub user_id: Option<u64>,
    pub skill_id: Option<u16>,
    pub item_id: Option<u64>,
}

#[derive(Deserialize)]
pub struct UserEquipmentCreateQuery {
    pub user_id: u64,
    pub skill_id: u16,
    pub item_id: u64,
    pub count: u32,
}

#[derive(Deserialize)]
pub struct UserEquipmentEditQuery {
    pub count: u32,
}

pub async fn user_equipment_list(
    State(state): State<AppState>,
    session: UserSession,
    Query(query): Query<UserEquipmentListQuery>,
) -> Result<Json<Vec<Equipment>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)?;

    let equipments = crate::db::inventory::user_equipment_list(conn, query.user_id, query.skill_id, query.item_id)?;
    Ok(Json(equipments))
}

pub async fn user_equipment_info(
    State(state): State<AppState>,
    session: UserSession,
    Path(equipment_id): Path<u64>,
) -> Result<Json<Equipment>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)?;

    let equipment = crate::db::inventory::user_equipment_info(conn, equipment_id)?;

    Ok(Json(equipment))
}

pub async fn user_equipment_create(
    State(state): State<AppState>,
    session: UserSession,
    Query(query): Query<UserEquipmentCreateQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::user_equipment_create(conn, query.user_id, query.skill_id, query.item_id, query.count)?;
    Ok(())
}

pub async fn user_equipment_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(equipment_id): Path<u64>,
    Query(query): Query<UserEquipmentEditQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::user_equipment_edit(conn, equipment_id, query.count)?;
    Ok(())
}

pub async fn user_equipment_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(equipment_id): Path<u64>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::user_equipment_delete(conn, equipment_id)?;
    Ok(())
}
