use rocket::serde::json::Json;

use crate::common::{Team, User};
use crate::error::Error;
use crate::session::UserSession;

/* ROUTES */

#[rocket::get("/admin/team_list")]
pub fn team_list(session: UserSession) -> Result<Json<Vec<Team>>, Error> {
    if !session.right.admin_teams {
        return Err(Error::RightTeamMissing);
    };

    match crate::db_team::list_teams() {
        None => Err(Error::DatabaseError),
        Some(teams) => Ok(Json(teams)),
    }
}

#[rocket::post("/admin/team_create", format = "application/json", data = "<team>")]
pub fn team_create(session: UserSession, team: Json<Team>) -> Result<String, Error> {
    if !session.right.admin_teams {
        return Err(Error::RightTeamMissing);
    };

    let team_id = crate::db_team::create_team(&team)?;
    Ok(team_id.to_string())
}

#[rocket::post("/admin/team_edit?<team_id>", format = "application/json", data = "<team>")]
pub fn team_edit(session: UserSession, team_id: u32, team: Json<Team>) -> Result<(), Error> {
    if !session.right.admin_teams {
        return Err(Error::RightTeamMissing);
    };

    match crate::db_team::edit_team(&team_id, &team) {
        None => Err(Error::DatabaseError),
        Some(..) => Ok(()),
    }
}

#[rocket::head("/admin/team_delete?<team_id>")]
pub fn team_delete(session: UserSession, team_id: u32) -> Result<(), Error> {
    if !session.right.admin_teams {
        return Err(Error::RightTeamMissing);
    };

    match crate::db_team::delete_team(&team_id) {
        None => Err(Error::DatabaseError),
        Some(..) => Ok(()),
    }
}

#[rocket::get("/admin/team_member_list?<team_id>")]
pub fn team_member_list(session: UserSession, team_id: u32) -> Result<Json<Vec<User>>, Error> {
    if !session.right.admin_teams {
        return Err(Error::RightTeamMissing);
    };

    match crate::db_team::list_team_members(team_id) {
        None => Err(Error::DatabaseError),
        Some(users) => Ok(Json(users)),
    }
}

#[rocket::head("/admin/team_member_add?<team_id>&<user_id>")]
pub fn team_member_add(session: UserSession, team_id: u32, user_id: u32) -> Result<(), Error> {
    if !session.right.admin_teams {
        return Err(Error::RightTeamMissing);
    };

    match crate::db_team::add_team_member(&team_id, &user_id) {
        None => Err(Error::DatabaseError),
        Some(..) => Ok(()),
    }

    // TODO: remove/add permissions of currently logged-in users
}

#[rocket::head("/admin/team_member_remove?<team_id>&<user_id>")]
pub fn team_member_remove(session: UserSession, team_id: u32, user_id: u32) -> Result<(), Error> {
    if !session.right.admin_teams {
        return Err(Error::RightTeamMissing);
    };

    match crate::db_team::remove_team_member(&team_id, &user_id) {
        None => Err(Error::DatabaseError),
        Some(..) => Ok(()),
    }

    // TODO: remove/add permissions of currently logged-in users
}
