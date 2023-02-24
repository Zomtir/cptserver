use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::session::{UserSession};
use crate::common::{Term};

#[rocket::get("/admin/term_list?<user_id>")]
pub fn term_list(session: UserSession, user_id: Option<i64>) -> Result<Json<Vec<Term>>, ApiError> {
    if !session.right.admin_term {return Err(ApiError::RIGHT_NO_TERM)};

    match crate::db_term::list_terms(user_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(terms) => Ok(Json(terms)),
    }
}

#[rocket::post("/term_create", format = "application/json", data = "<term>")]
pub fn term_create(session: UserSession, term: Json<Term>) -> Result<String, ApiError> {
    if !session.right.admin_term {return Err(ApiError::RIGHT_NO_TERM)};

    let id = crate::db_term::create_term(&term)?;
    Ok(id.to_string())
}

#[rocket::post("/term_edit?<term_id>", format = "application/json", data = "<term>")]
pub fn term_edit(session: UserSession, term_id: i64, term: Json<Term>) -> Result<(), ApiError> {
    if !session.right.admin_term {return Err(ApiError::RIGHT_NO_TERM)};

    match crate::db_term::edit_term(term_id, &term) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/term_delete?<term_id>")]
pub fn term_delete(session: UserSession, term_id: i64) -> Result<(), ApiError> {
    if !session.right.admin_term {return Err(ApiError::RIGHT_NO_TERM)};

    match crate::db_term::delete_term(term_id) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}
