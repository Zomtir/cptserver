use rocket::serde::json::Json;

use crate::common::{Right, User};
use crate::error::Error;
use crate::session::{Credential, UserSession};

/*
 * ROUTES
 */

#[rocket::get("/regular/user_info")]
pub fn user_info(session: UserSession) -> Result<Json<User>, Error> {
    let user = crate::db_user::get_user_detailed(session.user.id)?;
    Ok(Json(user))
}

#[rocket::get("/regular/user_right")]
pub fn user_right(session: UserSession) -> Json<Right> {
    Json(Right {
        admin_competence: session.right.admin_competence,
        admin_courses: session.right.admin_courses,
        admin_inventory: session.right.admin_inventory,
        admin_event: session.right.admin_event,
        admin_teams: session.right.admin_teams,
        admin_term: session.right.admin_term,
        admin_users: session.right.admin_users,
    })
}

#[rocket::post("/regular/user_password", format = "application/json", data = "<credit>")]
pub fn user_password(session: UserSession, credit: Json<Credential>) -> Result<(), Error> {
    crate::db_user::edit_user_password(session.user.id, &credit.password, &credit.salt)?;
    Ok(())
}

#[rocket::get("/regular/user_list")]
pub fn user_list(_session: UserSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db_user::list_user(Some(true))?;
    Ok(Json(users))
}
