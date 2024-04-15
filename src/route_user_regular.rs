use rocket::serde::json::Json;

use crate::common::{Right, User};
use crate::error::Error;
use crate::session::{Credential, UserSession};

/*
 * ROUTES
 */

#[rocket::get("/regular/user_info")]
pub fn user_info(session: UserSession) -> Result<Json<User>, Error> {
    let user = crate::db_user::user_info(session.user.id)?;
    Ok(Json(user))
}

#[rocket::get("/regular/user_right")]
pub fn user_right(session: UserSession) -> Json<Right> {
    Json(Right {
        right_club_write: session.right.right_club_write,
        right_club_read: session.right.right_club_read,
        right_competence_write: session.right.right_competence_write,
        right_competence_read: session.right.right_competence_read,
        right_course_write: session.right.right_course_write,
        right_course_read: session.right.right_course_read,
        right_event_write: session.right.right_event_write,
        right_event_read: session.right.right_event_read,
        right_inventory_write: session.right.right_inventory_write,
        right_inventory_read: session.right.right_inventory_read,
        right_location_write: session.right.right_location_write,
        right_location_read: session.right.right_location_read,
        right_team_write: session.right.right_team_write,
        right_team_read: session.right.right_team_read,
        right_user_write: session.right.right_user_write,
        right_user_read: session.right.right_user_read,
    })
}

#[rocket::post("/regular/user_password", format = "application/json", data = "<credit>")]
pub fn user_password(session: UserSession, credit: Json<Credential>) -> Result<(), Error> {
    crate::db_user::user_password_edit(session.user.id, &credit.password, &credit.salt)?;
    Ok(())
}

#[rocket::get("/regular/user_list")]
pub fn user_list(_session: UserSession) -> Result<Json<Vec<User>>, Error> {
    let users = crate::db_user::user_list(Some(true))?;
    Ok(Json(users))
}
