use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::{Possession, WebBool};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

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
