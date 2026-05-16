use rocket::serde::json::Json;

use crate::common::Term;
use crate::error::Result;
use crate::session::UserSession;

#[rocket::get("/admin/term_list?<club_id>&<user_id>")]
pub fn term_list(session: UserSession, club_id: Option<u32>, user_id: Option<u32>) -> Result<Json<Vec<Term>>> {
    crate::permission::require_right(session.right.right_club_read)?;

    let conn = &mut crate::utils::db::get_db_conn()?;
    let terms = crate::db::club::term_list(conn, club_id, user_id, None)?;
    Ok(Json(terms))
}

#[rocket::get("/admin/term_info?<term_id>")]
pub fn term_info(session: UserSession, term_id: u32) -> Result<Json<Term>> {
    crate::permission::require_right(session.right.right_club_read)?;

    let conn = &mut crate::utils::db::get_db_conn()?;
    let term = crate::db::club::term_info(conn, term_id)?;
    Ok(Json(term))
}

#[rocket::post("/admin/term_create", format = "application/json", data = "<term>")]
pub fn term_create(session: UserSession, term: Json<Term>) -> Result<String> {
    crate::permission::require_right(session.right.right_club_write)?;

    let conn = &mut crate::utils::db::get_db_conn()?;
    let id = crate::db::club::term_create(conn, &term)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/term_edit?<term_id>", format = "application/json", data = "<term>")]
pub fn term_edit(session: UserSession, term_id: i64, term: Json<Term>) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;

    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::db::club::term_edit(conn, term_id, &term)?;
    Ok(())
}

#[rocket::head("/admin/term_delete?<term_id>")]
pub fn term_delete(session: UserSession, term_id: i64) -> Result<()> {
    crate::permission::require_right(session.right.right_club_write)?;

    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::db::club::term_delete(conn, term_id)?;
    Ok(())
}
