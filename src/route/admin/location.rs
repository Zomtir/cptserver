use rocket::serde::json::Json;

use crate::common::Location;
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/admin/location_list")]
pub fn location_list(session: UserSession) -> Result<Json<Vec<Location>>, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_location_read {
        return Err(Error::RightLocationMissing);
    };

    let locations = crate::db::location::location_list(conn)?;
    Ok(Json(locations))
}

#[rocket::post("/admin/location_create", format = "application/json", data = "<location>")]
pub fn location_create(session: UserSession, location: Json<Location>) -> Result<String, Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_location_write {
        return Err(Error::RightLocationMissing);
    };

    let id = crate::db::location::location_create(conn, &location)?;
    Ok(id.to_string())
}

#[rocket::post(
    "/admin/location_edit?<location_id>",
    format = "application/json",
    data = "<location>"
)]
pub fn location_edit(session: UserSession, location_id: u32, location: Json<Location>) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_location_write {
        return Err(Error::RightLocationMissing);
    };

    crate::db::location::location_edit(conn, location_id, &location)?;
    Ok(())
}

#[rocket::head("/admin/location_delete?<location_id>")]
pub fn location_delete(session: UserSession, location_id: u32) -> Result<(), Error> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_location_write {
        return Err(Error::RightLocationMissing);
    };

    crate::db::location::location_delete(conn, location_id)?;
    Ok(())
}
