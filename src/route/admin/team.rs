use rocket::serde::json::Json;

use crate::common::{Right, Team, User};
use crate::error::Error;
use crate::session::UserSession;

/* ROUTES */

#[rocket::get("/admin/team_list")]
pub fn team_list(session: UserSession) -> Result<Json<Vec<Team>>, Error> {
    if !session.right.right_team_read {
        return Err(Error::RightTeamMissing);
    };

    let teams = crate::db::team::team_list()?;
    Ok(Json(teams))
}

#[rocket::post("/admin/team_create", format = "application/json", data = "<team>")]
pub fn team_create(session: UserSession, team: Json<Team>) -> Result<String, Error> {
    if !session.right.right_team_write {
        return Err(Error::RightTeamMissing);
    };

    let team_id = crate::db::team::team_create(&team)?;
    Ok(team_id.to_string())
}

#[rocket::post("/admin/team_edit?<team_id>", format = "application/json", data = "<team>")]
pub fn team_edit(session: UserSession, team_id: u32, team: Json<Team>) -> Result<(), Error> {
    if !session.right.right_team_write {
        return Err(Error::RightTeamMissing);
    };

    crate::db::team::team_edit(&team_id, &team)?;
    Ok(())
}

#[rocket::post("/admin/team_right_edit?<team_id>", format = "application/json", data = "<right>")]
pub fn team_right_edit(session: UserSession, team_id: u32, right: Json<Right>) -> Result<(), Error> {
    if !session.right.right_team_write {
        return Err(Error::RightTeamMissing);
    };

    crate::db::team::team_right_edit(&team_id, &right)?;
    Ok(())
}

#[rocket::head("/admin/team_delete?<team_id>")]
pub fn team_delete(session: UserSession, team_id: u32) -> Result<(), Error> {
    if !session.right.right_team_write {
        return Err(Error::RightTeamMissing);
    };

    crate::db::team::team_delete(&team_id)?;
    Ok(())
}

#[rocket::get("/admin/team_member_list?<team_id>")]
pub fn team_member_list(session: UserSession, team_id: u32) -> Result<Json<Vec<User>>, Error> {
    if !session.right.right_team_read {
        return Err(Error::RightTeamMissing);
    };

    let users = crate::db::team::team_member_list(team_id)?;
    Ok(Json(users))
}

#[rocket::head("/admin/team_member_add?<team_id>&<user_id>")]
pub fn team_member_add(session: UserSession, team_id: u32, user_id: u32) -> Result<(), Error> {
    if !session.right.right_team_write {
        return Err(Error::RightTeamMissing);
    };

    crate::db::team::team_member_add(&team_id, &user_id)?;
    Ok(())

    // TODO: remove/add permissions of currently logged-in users
}

#[rocket::head("/admin/team_member_remove?<team_id>&<user_id>")]
pub fn team_member_remove(session: UserSession, team_id: u32, user_id: u32) -> Result<(), Error> {
    if !session.right.right_team_write {
        return Err(Error::RightTeamMissing);
    };

    crate::db::team::team_member_remove(&team_id, &user_id)?;
    Ok(())

    // TODO: remove/add permissions of currently logged-in users
}
