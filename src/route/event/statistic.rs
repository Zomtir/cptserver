use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

use crate::common::{Affiliation, Item, User};
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

#[derive(Deserialize)]
pub struct EventStatisticPacklistQuery {
    pub skill_id: u32,
}

pub async fn statistic_packlist(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<EventStatisticPacklistQuery>,
) -> Result<Json<Vec<(User, Item, u32, u32, u32)>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_read)?;

    let stats = crate::db::event::event_statistic_packlist(conn, event_id, query.skill_id)?;
    Ok(Json(stats))
}

#[derive(Deserialize)]
pub struct EventStatisticOrganisationQuery {
    pub organisation_id: u64,
}

pub async fn statistic_organisation(
    State(state): State<AppState>,
    session: UserSession,
    Path(event_id): Path<u64>,
    Query(query): Query<EventStatisticOrganisationQuery>,
) -> Result<Json<Vec<Affiliation>>> {
    let conn = &mut state.db.get_conn()?;
    crate::permission::require_right(session.right.right_event_read)?;

    let stats = crate::db::event::event_statistic_organisation(conn, event_id, query.organisation_id)?;
    Ok(Json(stats))
}
