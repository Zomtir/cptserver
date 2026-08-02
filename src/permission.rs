use crate::error::{Error, ErrorKind, Result};
use mysql::PooledConn;

pub fn promote_user_to_admin(conn: &mut PooledConn) -> Result<Option<String>> {
    // Check if an admin user is configured
    let admin_key = match crate::config::ADMIN_USER() {
        Some(key) => key,
        None => return Ok(None),
    };

    // If admin user is missing, create him
    if crate::db::user::user_created_true(conn, admin_key)?.is_none() {
        let mut user = crate::common::User::from_info(
            0,
            admin_key.into(),
            "Placeholder".to_string(),
            "Placeholder".to_string(),
            None,
        );
        crate::db::user::user_create(conn, &mut user)?;
    }

    // Elevate the user to admin
    Ok(Some(admin_key.into()))
}

pub fn require_event_moderator(conn: &mut PooledConn, event_id: u64, user_id: u64) -> Result<()> {
    if crate::db::event::event_moderator_true(conn, event_id, user_id)? {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Permission,
            "User is not a moderator of the event",
        ))
    }
}

pub fn require_course_moderator(conn: &mut PooledConn, course_id: u32, user_id: u64) -> Result<()> {
    if crate::db::course::moderator::course_moderator_true(conn, course_id, user_id)? {
        Ok(())
    } else {
        Err(Error::new(
            ErrorKind::Permission,
            "User is not a moderator of the course",
        ))
    }
}

pub fn require_event_owner(conn: &mut PooledConn, event_id: u64, user_id: u64) -> Result<()> {
    if crate::db::event::owner::event_owner_true(conn, event_id, user_id)? {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Permission, "User is not an owner of the event"))
    }
}

pub fn require_right(permission: bool) -> Result<()> {
    if permission {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Permission, "User has insufficient rights"))
    }
}
