use rocket::serde::json::Json;

use crate::common::TermDiscipline;
use crate::error::Result;
use crate::session::UserSession;

#[rocket::post(
    "/admin/term_discipline_create?<term_id>",
    format = "application/json",
    data = "<term_discipline>"
)]
pub fn term_discipline_create(
    session: UserSession,
    term_id: u32,
    term_discipline: Json<TermDiscipline>,
) -> Result<String> {
    crate::permission::require_right(session.right.right_club_write)?;

    let conn = &mut crate::utils::db::get_db_conn()?;
    let id = crate::db::club::term_discipline_create(conn, term_id, &term_discipline)?;
    Ok(id.to_string())
}

#[rocket::post(
    "/admin/term_discipline_edit?<term_discipline_id>",
    format = "application/json",
    data = "<term_discipline>"
)]
pub fn term_discipline_edit(
    session: UserSession,
    term_discipline_id: u32,
    term_discipline: Json<TermDiscipline>,
) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;

    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::db::club::term_discipline_edit(conn, term_discipline_id, &term_discipline)?;
    Ok(())
}

#[rocket::head("/admin/term_discipline_delete?<term_discipline_id>")]
pub fn term_discipline_delete(session: UserSession, term_discipline_id: u32) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;

    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::db::club::term_discipline_delete(conn, term_discipline_id)?;
    Ok(())
}
