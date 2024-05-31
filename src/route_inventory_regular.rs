use rocket::serde::json::Json;

use crate::common::{Possession, WebBool};
use crate::error::Error;
use crate::session::UserSession;

#[rocket::get("/regular/possession_list?<owned>&<club_id>")]
pub fn possession_list(
    session: UserSession,
    owned: Option<WebBool>,
    club_id: Option<u32>,
) -> Result<Json<Vec<Possession>>, Error> {
    let possessions = crate::db_inventory::possession_list(Some(session.user.id), owned.map(|b| b.to_bool()), club_id)?;
    Ok(Json(possessions))
}
