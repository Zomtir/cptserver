use axum::extract::State;
use axum::Json;

use crate::common::{ItemCategory, Possession, WebBool};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub fn possession_list(
    State(state): State<AppState>,
    session: UserSession,
    owned: Option<WebBool>,
    club_id: Option<u32>,
) -> Result<Json<Vec<Possession>>> {
    let conn = &mut state.db.get_conn()?;
    let possessions =
        crate::db::inventory::possession_list(conn, Some(session.user.id), None, owned.map(|b| b.to_bool()), club_id)?;
    Ok(Json(possessions))
}

pub fn itemcat_list(State(state): State<AppState>, _session: UserSession) -> Result<Json<Vec<ItemCategory>>> {
    let conn = &mut state.db.get_conn()?;
    let itemcats = crate::db::inventory::itemcat_list(conn)?;
    Ok(Json(itemcats))
}
