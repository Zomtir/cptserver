use axum::Json;

use crate::common::Term;
use crate::error::Result;
use crate::session::UserSession;

pub fn term_list(session: UserSession, club_id: Option<u32>, user_id: Option<u32>) -> Result<Json<Vec<Term>>> {
    crate::permission::require_right(session.right.right_club_read)?;

    let conn = &mut crate::utils::db::get_db_conn()?;
    let terms = crate::db::club::term_list(conn, club_id, user_id, None)?;
    Ok(Json(terms))
}

pub fn term_info(session: UserSession, term_id: u64) -> Result<Json<Term>> {
    crate::permission::require_right(session.right.right_club_read)?;

    let conn = &mut crate::utils::db::get_db_conn()?;
    let term = crate::db::club::term_info(conn, term_id)?;
    Ok(Json(term))
}

pub fn term_create(session: UserSession, term: Json<Term>) -> Result<String> {
    crate::permission::require_right(session.right.right_club_write)?;

    let conn = &mut crate::utils::db::get_db_conn()?;
    let id = crate::db::club::term_create(conn, &term)?;
    Ok(id.to_string())
}

pub fn term_edit(session: UserSession, term_id: u32, term: Json<Term>) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;

    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::db::club::term_edit(conn, term_id, &term)?;
    Ok(())
}

pub fn term_delete(session: UserSession, term_id: u32) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;

    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::db::club::term_delete(conn, term_id)?;
    Ok(())
}
