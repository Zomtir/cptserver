use axum::extract::{Path, Query, State};
use axum::Json;

use crate::common::{Affiliation, Organisation};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AffiliationListQuery {
    pub user_id: Option<u64>,
    pub organisation_id: Option<u32>,
}

pub async fn organisation_list(State(state): State<AppState>) -> Result<Json<Vec<Organisation>>> {
    let conn = &mut state.db.get_conn()?;
    let organisations = crate::db::organisation::organisation_list(conn)?;
    Ok(Json(organisations))
}

pub async fn organisation_info(
    State(state): State<AppState>,
    session: UserSession,
    Path(organisation_id): Path<u32>,
) -> Result<Json<Organisation>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_read)?;

    let organisation = crate::db::organisation::organisation_info(conn, organisation_id)?;
    Ok(Json(organisation))
}

pub async fn organisation_create(
    State(state): State<AppState>,
    session: UserSession,
    organisation: Json<Organisation>,
) -> Result<String> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    let id = crate::db::organisation::organisation_create(conn, &organisation)?;
    Ok(id.to_string())
}

pub async fn organisation_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path(organisation_id): Path<u32>,
    organisation: Json<Organisation>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::organisation_edit(conn, organisation_id, &organisation)?;
    Ok(())
}

pub async fn organisation_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path(organisation_id): Path<u32>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::organisation_delete(conn, organisation_id)?;
    Ok(())
}

pub async fn affiliation_list(
    State(state): State<AppState>,
    session: UserSession,
    Query(query): Query<AffiliationListQuery>,
) -> Result<Json<Vec<Affiliation>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_read)?;

    let affiliations = crate::db::organisation::affiliation_list(conn, query.user_id, query.organisation_id)?;
    Ok(Json(affiliations))
}

pub async fn affiliation_info(
    State(state): State<AppState>,
    session: UserSession,
    Path((organisation_id, user_id)): Path<(u32, u64)>,
) -> Result<Json<Option<Affiliation>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_read)?;

    let affiliation = crate::db::organisation::affiliation_info(conn, user_id, organisation_id)?;
    Ok(Json(affiliation))
}

pub async fn affiliation_create(
    State(state): State<AppState>,
    session: UserSession,
    Path((organisation_id, user_id)): Path<(u32, u64)>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::affiliation_create(conn, user_id, organisation_id)?;
    Ok(())
}

pub async fn affiliation_edit(
    State(state): State<AppState>,
    session: UserSession,
    Path((organisation_id, user_id)): Path<(u32, u64)>,
    affiliation: Json<Affiliation>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::affiliation_edit(conn, user_id, organisation_id, &affiliation)?;
    Ok(())
}

pub async fn affiliation_delete(
    State(state): State<AppState>,
    session: UserSession,
    Path((organisation_id, user_id)): Path<(u32, u64)>,
) -> Result<()> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_organisation_write)?;

    crate::db::organisation::affiliation_delete(conn, user_id, organisation_id)?;
    Ok(())
}
