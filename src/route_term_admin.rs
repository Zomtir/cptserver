// let user_term : chrono::NaiveDate = row.take("term").unwrap();
// if chrono::Date::<chrono::Utc>::from_utc(user_term, chrono::Utc) < chrono::Utc::today() {
//     return Err(ApiError::USER_EXPIRED);
// }

// let params = params! {
//     "date_today" => chrono::Utc::today().to_string(),
// };

// #[serde(with = "crate::clock::date_format")]
// pub term_begin: chrono::NaiveDate,
// pub term_end: chrono::NaiveDate,

use rocket::serde::json::Json;

use crate::api::ApiError;
use crate::session::{UserSession};
use crate::common::{Term};

#[rocket::get("/term_list?<user_id>")]
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

    match crate::db_term::create_term(&term) {
        None => Err(ApiError::DB_CONFLICT),
        Some(id) => Ok(id.to_string()),
    }
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
