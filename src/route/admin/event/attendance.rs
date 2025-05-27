use rocket::serde::json::Json;

use crate::common::User;
use crate::error::{ErrorKind, Result};
use crate::session::UserSession;

#[rocket::get("/admin/event_attendance_registration_list?<event_id>&<role>")]
pub fn registration_list(session: UserSession, event_id: u64, role: String) -> Result<Json<Vec<User>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_read {
        return Err(ErrorKind::RightEventMissing);
    };

    let users = crate::db::event::attendance::event_attendance_registration_list(conn, event_id, role)?;
    Ok(Json(users))
}

#[rocket::get("/admin/event_attendance_filter_list?<event_id>&<role>")]
pub fn filter_list(session: UserSession, event_id: u64, role: String) -> Result<Json<Vec<(User, bool)>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_read {
        return Err(ErrorKind::RightEventMissing);
    };

    let filters = crate::db::event::attendance::event_attendance_filter_list(conn, event_id, role)?;
    Ok(Json(filters))
}

#[rocket::head("/admin/event_attendance_filter_edit?<event_id>&<user_id>&<role>&<access>")]
pub fn filter_edit(session: UserSession, event_id: u64, user_id: u64, role: String, access: bool) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_write {
        return Err(ErrorKind::RightEventMissing);
    };

    crate::db::event::attendance::event_attendance_filter_edit(conn, event_id, user_id, role, access)?;
    Ok(())
}

#[rocket::head("/admin/event_attendance_filter_remove?<event_id>&<user_id>&<role>")]
pub fn filter_remove(session: UserSession, event_id: u64, user_id: u64, role: String) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_write {
        return Err(ErrorKind::RightEventMissing);
    };

    crate::db::event::attendance::event_attendance_filter_remove(conn, event_id, user_id, role)?;
    Ok(())
}

#[rocket::get("/admin/event_attendance_presence_pool?<event_id>&<role>")]
pub fn presence_pool(session: UserSession, event_id: u64, role: String) -> Result<Json<Vec<User>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_read {
        return Err(ErrorKind::RightEventMissing);
    };

    let users = crate::db::event::attendance::event_attendance_presence_pool(conn, event_id, &role, true)?;
    Ok(Json(users))
}

#[rocket::get("/admin/event_attendance_presence_list?<event_id>&<role>")]
pub fn presence_list(session: UserSession, event_id: u64, role: String) -> Result<Json<Vec<User>>> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_read {
        return Err(ErrorKind::RightEventMissing);
    };

    let users = crate::db::event::attendance::event_attendance_presence_list(conn, event_id, &role)?;
    Ok(Json(users))
}

#[rocket::head("/admin/event_attendance_presence_add?<event_id>&<user_id>&<role>")]
pub fn presence_add(session: UserSession, event_id: u64, user_id: u64, role: String) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_write {
        return Err(ErrorKind::RightEventMissing);
    };

    let pool = crate::db::event::attendance::event_attendance_presence_pool(conn, event_id, &role, true)?;

    if !pool.iter().any(|user| user.id == user_id) {
        return Err(ErrorKind::EventPresenceForbidden);
    }

    crate::db::event::attendance::event_attendance_presence_add(conn, event_id, user_id, &role)?;
    Ok(())
}

#[rocket::head("/admin/event_attendance_presence_remove?<event_id>&<user_id>&<role>")]
pub fn presence_remove(session: UserSession, event_id: u64, user_id: u64, role: String) -> Result<()> {
    let conn = &mut crate::utils::db::get_db_conn()?;
    if !session.right.right_event_write {
        return Err(ErrorKind::RightEventMissing);
    };

    crate::db::event::attendance::event_attendance_presence_remove(conn, event_id, user_id, &role)?;
    Ok(())
}
