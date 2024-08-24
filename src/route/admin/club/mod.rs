pub mod term;

use rocket::serde::json::Json;

use crate::common::{Club, Term, User, WebDate};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/club_list")]
pub fn club_list(session: UserSession) -> Result<Json<Vec<Club>>, Error> {
    if !session.right.right_club_read {
        return Err(Error::RightClubMissing);
    };

    let clubs = crate::db::club::club_list()?;
    Ok(Json(clubs))
}

#[rocket::post("/admin/club_create", format = "application/json", data = "<club>")]
pub fn club_create(session: UserSession, club: Json<Club>) -> Result<String, Error> {
    if !session.right.right_club_write {
        return Err(Error::RightClubMissing);
    };

    let id = crate::db::club::club_create(&club)?;
    Ok(id.to_string())
}

#[rocket::post("/admin/club_edit?<club_id>", format = "application/json", data = "<club>")]
pub fn club_edit(session: UserSession, club_id: u32, club: Json<Club>) -> Result<(), Error> {
    if !session.right.right_club_write {
        return Err(Error::RightClubMissing);
    };

    crate::db::club::club_edit(club_id, &club)?;
    Ok(())
}

#[rocket::head("/admin/club_delete?<club_id>")]
pub fn club_delete(session: UserSession, club_id: u32) -> Result<(), Error> {
    if !session.right.right_club_write {
        return Err(Error::RightClubMissing);
    };

    crate::db::club::club_delete(club_id)?;
    Ok(())
}

/* STATISTICS */

#[rocket::get("/admin/club_statistic_terms?<club_id>&<point_in_time>")]
pub fn statistic_terms(session: UserSession, club_id: u32, point_in_time: WebDate) -> Result<Json<Vec<Term>>, Error> {
    if !session.right.right_club_read {
        return Err(Error::RightClubMissing);
    };

    let terms = crate::db::club::term_list(Some(club_id), None, Some(point_in_time.to_naive()))?;
    Ok(Json(terms))
}

#[rocket::get("/admin/club_statistic_members?<club_id>&<point_in_time>")]
pub fn statistic_members(
    session: UserSession,
    club_id: u32,
    point_in_time: WebDate,
) -> Result<Json<Vec<(User, u32)>>, Error> {
    if !session.right.right_club_read {
        return Err(Error::RightClubMissing);
    };

    let leaderboard = crate::db::club::club_member_leaderboard(club_id, None, point_in_time.to_naive())?;
    Ok(Json(leaderboard))
}

#[rocket::get("/admin/club_statistic_team?<club_id>&<point_in_time>&<team_id>")]
pub fn statistic_team(
    session: UserSession,
    club_id: u32,
    point_in_time: WebDate,
    team_id: u32,
) -> Result<Json<Vec<User>>, Error> {
    if !session.right.right_club_read {
        return Err(Error::RightClubMissing);
    };

    let users = crate::db::club::club_team_comparison(club_id, team_id, point_in_time.to_naive())?;
    Ok(Json(users))
}

#[rocket::get("/admin/club_statistic_organisation?<club_id>&<point_in_time>")]
pub fn statistic_organisation(
    session: UserSession,
    club_id: u32,
    point_in_time: WebDate,
) -> Result<Json<Vec<User>>, Error> {
    if !session.right.right_club_read {
        return Err(Error::RightClubMissing);
    };

    let users = crate::db::club::club_member_organisation(club_id, None, point_in_time.to_naive())?;
    Ok(Json(users))
}
