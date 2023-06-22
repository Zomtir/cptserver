use rocket::serde::json::Json;

use crate::common::{Right, User};
use crate::error::Error;
use crate::session::{Credential, UserSession};

/*
 * ROUTES
 */

#[rocket::get("/member/user_info")]
pub fn user_info(session: UserSession) -> Json<User> {
    Json(User::from_info(
        session.user.id,
        session.user.key.unwrap(),
        session.user.firstname,
        session.user.lastname,
    ))
}

#[rocket::get("/member/user_right")]
pub fn user_right(session: UserSession) -> Json<Right> {
    Json(Right {
        admin_courses: session.right.admin_courses,
        admin_inventory: session.right.admin_inventory,
        admin_rankings: session.right.admin_rankings,
        admin_event: session.right.admin_event,
        admin_teams: session.right.admin_teams,
        admin_term: session.right.admin_term,
        admin_users: session.right.admin_users,
    })
}

#[rocket::post("/member/user_password", format = "application/json", data = "<credit>")]
pub fn user_password(session: UserSession, credit: Json<Credential>) -> Result<(), Error> {
    crate::db_user::edit_user_password(session.user.id, &credit.password, &credit.salt)?;
    Ok(())
}

#[rocket::get("/member/user_list")]
pub fn user_list(_session: UserSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db_user::list_user(Some(true))?;
    Ok(Json(users))
}
