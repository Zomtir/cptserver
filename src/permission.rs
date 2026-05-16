use crate::error::{Error, ErrorKind, Result};
use mysql::PooledConn;

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
