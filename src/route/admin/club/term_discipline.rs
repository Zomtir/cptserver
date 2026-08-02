use axum::extract::State;
use axum::Json;

use crate::common::TermDiscipline;
use crate::error::Result;
use crate::session::UserSession;
use crate::AppState;

pub fn term_discipline_create(
    State(state): State<AppState>,
    session: UserSession,
    term_id: u32,
    term_discipline: Json<TermDiscipline>,
) -> Result<String> {
    crate::permission::require_right(session.right.right_club_write)?;

    let conn = &mut state.db.get_conn()?;
    let id = crate::db::club::term_discipline_create(conn, term_id, &term_discipline)?;
    Ok(id.to_string())
}

pub fn term_discipline_edit(
    State(state): State<AppState>,
    session: UserSession,
    term_discipline_id: u32,
    term_discipline: Json<TermDiscipline>,
) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;

    let conn = &mut state.db.get_conn()?;
    crate::db::club::term_discipline_edit(conn, term_discipline_id, &term_discipline)?;
    Ok(())
}

pub fn term_discipline_delete(
    State(state): State<AppState>,
    session: UserSession,
    term_discipline_id: u32,
) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;

    let conn = &mut state.db.get_conn()?;
    crate::db::club::term_discipline_delete(conn, term_discipline_id)?;
    Ok(())
}
