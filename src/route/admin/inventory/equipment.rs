use rocket::serde::json::Json;

use crate::common::Equipment;
use crate::error::Result;
use crate::session::UserSession;

#[rocket::get("/admin/user_equipment_list?<user_id>&<skill_id>&<item_id>")]
pub fn user_equipment_list(
    session: UserSession,
    user_id: Option<u64>,
    skill_id: Option<u16>,
    item_id: Option<u64>,
) -> Result<Json<Vec<Equipment>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)?;

    let equipments = crate::db::inventory::user_equipment_list(conn, user_id, skill_id, item_id)?;
    Ok(Json(equipments))
}

#[rocket::get("/admin/user_equipment_info?<equipment_id>")]
pub fn user_equipment_info(session: UserSession, equipment_id: u64) -> Result<Json<Equipment>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)?;

    let equipment = crate::db::inventory::user_equipment_info(conn, equipment_id)?;

    Ok(Json(equipment))
}

#[rocket::head("/admin/user_equipment_create?<user_id>&<skill_id>&<item_id>&<count>")]
pub fn user_equipment_create(
    session: UserSession,
    user_id: u64,
    skill_id: u16,
    item_id: u64,
    count: u32,
) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::user_equipment_create(conn, user_id, skill_id, item_id, count)?;
    Ok(())
}

#[rocket::head("/admin/user_equipment_edit?<equipment_id>&<count>")]
pub fn user_equipment_edit(session: UserSession, equipment_id: u64, count: u32) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::user_equipment_edit(conn, equipment_id, count)?;
    Ok(())
}

#[rocket::head("/admin/user_equipment_delete?<equipment_id>")]
pub fn user_equipment_delete(session: UserSession, equipment_id: u64) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    crate::db::inventory::user_equipment_delete(conn, equipment_id)?;
    Ok(())
}
