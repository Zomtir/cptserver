use crate::common::Stock;
use crate::error::{Error, ErrorKind, Result};
use crate::session::UserSession;
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct StockListQuery {
    pub club_id: Option<u32>,
    pub item_id: Option<u32>,
}

#[derive(Deserialize)]
pub struct StockLoanQuery {
    pub user_id: u64,
}

pub async fn stock_list(
    State(state): State<AppState>,
    session: UserSession,
    Query(query): Query<StockListQuery>,
) -> Result<Json<Vec<Stock>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_read)?;

    let stocks = crate::db::inventory::stock_list(conn, query.club_id, query.item_id)?;
    Ok(Json(stocks))
}

pub async fn stock_create(State(state): State<AppState>, session: UserSession, stock: Json<Stock>) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    if stock.owned > 100 {
        return Err(Error::new(ErrorKind::Boundary, "Inventory stock limit reached"));
    }

    crate::db::inventory::stock_create(conn, stock.club.id, stock.item.id, &stock.storage, stock.owned)?;

    Ok(())
}

pub async fn stock_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(stock_id): Path<u64>,
    stock: Json<Stock>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    if stock.owned > 100 {
        return Err(Error::new(ErrorKind::Boundary, "Inventory stock limit reached"));
    }

    let db_stock = crate::db::inventory::stock_info(conn, stock_id)?;

    let delta = stock.owned as i64 - db_stock.owned as i64;

    // No change, useless request
    if delta == 0 && db_stock.storage == stock.storage {
        return Ok(());
    }

    // Do not remove loaned items
    if stock.owned < db_stock.loaned {
        return Err(Error::new(
            ErrorKind::Boundary,
            "Cannot reduce stock below loaned amount",
        ));
    }

    // Check if the client has a different loan information
    if db_stock.loaned != stock.loaned {
        return Err(Error::new(ErrorKind::Mismatch, "Loan information does not match"));
    }

    crate::db::inventory::stock_edit(conn, stock_id, &stock.storage, stock.owned, stock.loaned)?;

    Ok(())
}

pub async fn stock_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(stock_id): Path<u64>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    let stock = crate::db::inventory::stock_info(conn, stock_id)?;

    // Cannot delete a stock that is incomplete
    if stock.loaned > 0 {
        return Err(Error::new(
            ErrorKind::Conflict,
            "Cannot delete stock with outstanding loans",
        ));
    }

    crate::db::inventory::stock_delete(conn, stock_id)?;

    Ok(())
}

pub async fn item_loan(
    State(state): State<AppState>,
    session: UserSession,
    Path(stock_id): Path<u64>,
    Query(query): Query<StockLoanQuery>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    let stock = crate::db::inventory::stock_info(conn, stock_id)?;

    // No items available to loan
    if stock.owned <= stock.loaned {
        return Err(Error::new(ErrorKind::Boundary, "No items available to loan"));
    }

    crate::db::inventory::stock_edit(conn, stock_id, &stock.storage, stock.owned, stock.loaned + 1)?;
    crate::db::inventory::possession_create(
        conn,
        query.user_id,
        stock.item.id,
        chrono::Utc::now().date_naive(),
        false,
        Some(stock_id),
    )?;

    Ok(())
}

pub async fn item_return(
    State(state): State<AppState>,
    session: UserSession,
    Path(possession_id): Path<u64>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    let possession = crate::db::inventory::possession_info(conn, possession_id)?;
    let stock = crate::db::inventory::possession_ownership(conn, possession_id)?;

    let stock = match (possession.owned, stock) {
        // Cannot return items which are owned by a user
        (true, None) => {
            return Err(Error::new(
                ErrorKind::Conflict,
                "Cannot return items which are owned by a user",
            ))
        }
        // Invalid database state, belongs to user but has stock information
        (true, Some(_)) => return Err(Error::new(ErrorKind::Database, "Invalid database state")),
        // Invalid database state, does not belong to the user but is missing stock information
        (false, None) => return Err(Error::new(ErrorKind::Database, "Invalid database state")),
        // Does not belong to user, can be returned
        (false, Some(stock)) => stock,
    };

    // Should not happen, but make sure that there are loaned items that can be returned
    if stock.loaned < 1 {
        return Err(Error::new(
            ErrorKind::Database,
            "Trying to return an item that is not loaned",
        ));
    }

    crate::db::inventory::stock_edit(conn, stock.id, &stock.storage, stock.owned, stock.loaned - 1)?;
    crate::db::inventory::possession_delete(conn, possession_id)?;

    Ok(())
}

pub async fn item_handout(
    State(state): State<AppState>,
    session: UserSession,
    Path(possession_id): Path<u64>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    let possession = crate::db::inventory::possession_info(conn, possession_id)?;
    let stock = crate::db::inventory::possession_ownership(conn, possession_id)?;

    let stock = match (possession.owned, stock) {
        // Cannot hand out items that already belong to a user
        (true, None) => {
            return Err(Error::new(
                ErrorKind::Conflict,
                "Cannot hand out items that already belong to a user",
            ))
        }
        // Invalid database state, belongs to user but has stock information
        (true, Some(_)) => return Err(Error::new(ErrorKind::Database, "Invalid database state")),
        // Invalid database state, does not belong to the user but is missing stock information
        (false, None) => return Err(Error::new(ErrorKind::Database, "Invalid database state")),
        // Does not belong to user, can be handened out
        (false, Some(stock)) => stock,
    };

    // Should not happen, but make sure that there are loaned items that can be handed out
    if stock.owned < 1 || stock.loaned < 1 {
        return Err(Error::new(
            ErrorKind::Conflict,
            "Cannot hand out items that are not available",
        ));
    }

    crate::db::inventory::possession_edit(conn, possession_id, &possession, None)?;
    crate::db::inventory::stock_edit(conn, stock.id, &stock.storage, stock.owned - 1, stock.loaned - 1)?;

    Ok(())
}

pub async fn item_restock(
    State(state): State<AppState>,
    session: UserSession,
    Path((possession_id, stock_id)): Path<(u64, u64)>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_inventory_write)?;

    let possession = crate::db::inventory::possession_info(conn, possession_id)?;
    let stock = crate::db::inventory::stock_info(conn, stock_id)?;

    // Cannot restock items on a stock of a different item type
    if possession.item.id != stock.item.id {
        return Err(Error::new(ErrorKind::Mismatch, "Item type mismatch"));
    };

    // Cannot restock items one does not own
    if !possession.owned {
        return Err(Error::new(
            ErrorKind::Permission,
            "Cannot put items back on stock that are not owned",
        ));
    };

    crate::db::inventory::possession_edit(conn, possession_id, &possession, Some(stock_id))?;
    crate::db::inventory::stock_edit(conn, stock_id, &stock.storage, stock.owned + 1, stock.loaned + 1)?;

    Ok(())
}

