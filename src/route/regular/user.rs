use rocket::serde::json::Json;

use crate::common::{Right, User};
use crate::error::Error;
use crate::session::{Credential, UserSession};

/*
 * ROUTES
 */

#[rocket::get("/regular/user_info")]
pub fn user_info(session: UserSession) -> Result<Json<User>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let user = crate::db::user::user_info(conn, session.user.id)?;
    Ok(Json(user))
}

#[rocket::get("/regular/user_right")]
pub fn user_right(session: UserSession) -> Json<Right> {
    Json(session.right)
}

#[rocket::post("/regular/user_password", format = "application/json", data = "<credit>")]
pub fn user_password(session: UserSession, credit: Json<Credential>) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    crate::db::user::user_password_edit(conn, session.user.id, &credit.password, &credit.salt)?;
    Ok(())
}

#[rocket::get("/regular/user_list")]
pub fn user_list(_session: UserSession) -> Result<Json<Vec<User>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    let users = crate::db::user::user_list(conn, Some(true))?;
    Ok(Json(users))
}
