use rocket::serde::json::Json;

use crate::common::Term;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/term_list?<user_id>")]
pub fn term_list(session: UserSession, user_id: Option<u32>) -> Result<Json<Vec<Term>>, Error> {
    if !session.right.right_club_read {
        return Err(Error::RightClubMissing);
    };

    let terms = crate::db_term::term_list(None, user_id, None)?;
    Ok(Json(terms))
}

#[rocket::post("/admin/term_create", format = "application/json", data = "<term>")]
pub fn term_create(session: UserSession, term: Json<Term>) -> Result<String, Error> {
    if !session.right.right_club_write {
        return Err(Error::RightClubMissing);
    };

    let id = crate::db_term::term_create(&term)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/term_edit?<term_id>", format = "application/json", data = "<term>")]
pub fn term_edit(session: UserSession, term_id: i64, term: Json<Term>) -> Result<(), Error> {
    if !session.right.right_club_write {
        return Err(Error::RightClubMissing);
    };

    crate::db_term::term_edit(term_id, &term)?;
    Ok(())
}

#[rocket::head("/admin/term_delete?<term_id>")]
pub fn term_delete(session: UserSession, term_id: i64) -> Result<(), Error> {
    if !session.right.right_club_write {
        return Err(Error::RightClubMissing);
    };

    crate::db_term::term_delete(term_id)?;
    Ok(())
}
