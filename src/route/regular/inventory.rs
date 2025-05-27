use rocket::serde::json::Json;

use crate::common::{ItemCategory, Possession, WebBool};
use crate::error::Result;
use crate::session::UserSession;

#[rocket::get("/regular/possession_list?<owned>&<club_id>")]
pub fn possession_list(
    session: UserSession,
    owned: Option<WebBool>,
    club_id: Option<u32>,
) -> Result<Json<Vec<Possession>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let possessions =
        crate::db::inventory::possession_list(conn, Some(session.user.id), None, owned.map(|b| b.to_bool()), club_id)?;
    Ok(Json(possessions))
}

#[rocket::get("/regular/itemcat_list")]
pub fn itemcat_list(_session: UserSession) -> Result<Json<Vec<ItemCategory>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let itemcats = crate::db::inventory::itemcat_list(conn)?;
    Ok(Json(itemcats))
}
