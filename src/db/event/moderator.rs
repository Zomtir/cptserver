use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::error::ErrorKind;

pub fn event_moderator_true(conn: &mut PooledConn, event_id: u64, user_id: u64) -> Result<bool, ErrorKind> {
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM course_moderators cm
        JOIN events e ON e.course_id = cm.course_id
        WHERE e.event_id = :event_id AND cm.user_id = :user_id;",
    )?;
    let params = params! {
        "event_id" => event_id,
        "user_id" => user_id,
    };

    match conn.exec_first::<u64, _, _>(&stmt, &params)? {
        None => Ok(false),
        Some(count) => Ok(count == 1),
    }
}
