use rocket::serde::json::Json;

use crate::common::Club;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/club_list")]
pub fn club_list(
    session: UserSession,
) -> Result<Json<Vec<Club>>, Error> {
    if !session.right.admin_term {
        return Err(Error::RightTermMissing);
    };

    let clubs = crate::db_club::club_list()?;
    Ok(Json(clubs))
}

#[rocket::post("/admin/club_create", format = "application/json", data = "<club>")]
pub fn club_create(session: UserSession, club: Json<Club>) -> Result<String, Error> {
    if !session.right.admin_term {
        return Err(Error::RightTermMissing);
    };

    let id = crate::db_club::club_create(&club)?;
    Ok(id.to_string())
}

#[rocket::post(
    "/admin/club_edit?<club_id>",
    format = "application/json",
    data = "<club>"
)]
pub fn club_edit(session: UserSession, club_id: u32, club: Json<Club>) -> Result<(), Error> {
    if !session.right.admin_term {
        return Err(Error::RightTermMissing);
    };

    crate::db_club::club_edit(club_id, &club)?;
    Ok(())
}

#[rocket::head("/admin/club_delete?<club_id>")]
pub fn club_delete(session: UserSession, club_id: u32) -> Result<(), Error> {
    if !session.right.admin_term {
        return Err(Error::RightTermMissing);
    };

    crate::db_club::club_delete(club_id)?;
    Ok(())
}
