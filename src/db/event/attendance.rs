use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Confirmation, User};
use crate::error::Error;

/* REGISTRATIONS */

pub fn event_attendance_registration_list(
    conn: &mut PooledConn,
    event_id: u64,
    role: String,
) -> Result<Vec<User>, Error> {
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM event_attendance_registrations
        JOIN users u ON u.user_id = event_attendance_registrations.user_id
        WHERE event_id = :event_id
        AND role = :role;",
    )?;
    let params = params! {
        "event_id" => event_id,
        "role" => role,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn event_attendance_registration_info(
    conn: &mut PooledConn,
    event_id: u64,
    user_id: u64,
    role: String,
) -> Result<Confirmation, Error> {
    let stmt = conn.prep(
        "SELECT r.status
        FROM event_attendance_registrations r
        WHERE r.event_id = :event_id AND r.user_id = :user_id AND r.role = :role;",
    )?;

    let params = params! {
        "event_id" => event_id,
        "user_id" => user_id,
        "role" => role,
    };

    let row = conn.exec_first::<String, _, _>(&stmt, &params)?;

    match row {
        Some(status) => Ok(status.parse()?),
        None => Ok(Confirmation::Null),
    }
}

pub fn event_attendance_registration_edit(
    conn: &mut PooledConn,
    event_id: u64,
    user_id: u64,
    role: String,
    status: Confirmation,
) -> Result<(), Error> {
    let stmt = conn.prep(
        "INSERT INTO event_attendance_registrations (event_id, user_id, role, status)
        VALUES (:event_id, :user_id, :role, :status)
        ON DUPLICATE KEY UPDATE status = :status;",
    )?;
    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
        "role" => &role,
        "status" => &status,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_attendance_registration_remove(
    conn: &mut PooledConn,
    event_id: u64,
    user_id: u64,
    role: String,
) -> Result<(), Error> {
    let stmt = conn.prep(
        "DELETE FROM event_attendance_registrations
        WHERE event_id = :event_id AND user_id = :user_id AND role = :role;",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
        "role" => &role,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* FILTER */

pub fn event_attendance_filter_list(
    conn: &mut PooledConn,
    event_id: u64,
    role: String,
) -> Result<Vec<(User, bool)>, Error> {
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname, ef.access
        FROM event_attendance_filters ef
        JOIN users u ON u.user_id = ef.user_id
        WHERE event_id = :event_id AND role = :role;",
    )?;
    let params = params! {
        "event_id" => event_id,
        "role" => role,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname, access)| {
        (
            User::from_info(user_id, user_key, firstname, lastname, nickname),
            access,
        )
    };

    let filters = conn.exec_map(&stmt, &params, &map)?;
    Ok(filters)
}

pub fn event_attendance_filter_edit(
    conn: &mut PooledConn,
    event_id: u64,
    user_id: u64,
    role: String,
    access: bool,
) -> Result<(), Error> {
    let stmt = conn.prep(
        "INSERT INTO event_attendance_filters (event_id, user_id, role, access)
        VALUES (:event_id, :user_id, :role, :access)
        ON DUPLICATE KEY UPDATE access = :access;",
    )?;
    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
        "role" => role,
        "access" => &access,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_attendance_filter_remove(
    conn: &mut PooledConn,
    event_id: u64,
    user_id: u64,
    role: String,
) -> Result<(), Error> {
    let stmt = conn.prep(
        "DELETE FROM event_attendance_filters
        WHERE event_id = :event_id AND user_id = :user_id AND role = :role;",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
        "role" => &role,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* PRESENCE */

pub fn event_attendance_presence_pool(
    conn: &mut PooledConn,
    event_id: u64,
    role: &str,
    access: bool,
) -> Result<Vec<User>, Error> {
    let stmt = conn.prep(
        "SELECT users.user_id, users.user_key, users.firstname, users.lastname, users.nickname
        FROM users
        INNER JOIN (
            SELECT er.user_id, NULL sieves_access, NULL AS filters_access, TRUE AS registration_access
            FROM event_attendance_registrations as er
            WHERE er.event_id = :event_id AND er.role = :role
            AND (er.status = 'POSITIVE' OR er.status = 'NEUTRAL')
            UNION ALL
            SELECT tm.user_id, MIN(sieves.access) AS sieves_access, NULL AS filters_access, NULL AS registration_access
            FROM course_attendance_sieves as sieves
            JOIN teams ON teams.team_id = sieves.team_id
            JOIN team_members tm ON teams.team_id = tm.team_id
            JOIN events ON events.course_id = sieves.course_id
            WHERE events.event_id = :event_id AND sieves.role = :role
			GROUP BY tm.user_id
            UNION ALL
            SELECT filters.user_id, NULL AS sieves_access, filters.access AS filters_access, NULL AS registration_access
            FROM event_attendance_filters as filters
            WHERE filters.event_id = :event_id AND filters.role = :role
        ) AS pool ON pool.user_id = users.user_id
        GROUP BY user_id
        HAVING COALESCE(MAX(filters_access), MAX(sieves_access), MAX(registration_access)) = :access;",
    )?;

    let params = params! {
        "event_id" => event_id,
        "role" => role,
        "access" => access,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn event_attendance_presence_list(conn: &mut PooledConn, event_id: u64, role: &str) -> Result<Vec<User>, Error> {
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM event_attendance_presences ep
        JOIN users u ON u.user_id = ep.user_id
        WHERE event_id = :event_id AND :role = role;",
    )?;
    let params = params! {
        "event_id" => event_id,
        "role" => role,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let users = conn.exec_map(&stmt, &params, &map)?;
    Ok(users)
}

pub fn event_attendance_presence_true(
    conn: &mut PooledConn,
    event_id: u64,
    user_id: u64,
    role: &str,
) -> Result<bool, Error> {
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM event_attendance_presences ep
        WHERE ep.event_id = :event_id AND ep.user_id = :user_id AND role = :role;",
    )?;

    let params = params! {
        "event_id" => event_id,
        "user_id" => user_id,
        "role" => role,
    };

    match conn.exec_first::<u32, _, _>(&stmt, &params)? {
        Some(0) => Ok(false),
        Some(1) => Ok(true),
        _ => Err(Error::DatabaseError),
    }
}

pub fn event_attendance_presence_add(
    conn: &mut PooledConn,
    event_id: u64,
    user_id: u64,
    role: &str,
) -> Result<(), Error> {
    let stmt = conn.prep(
        "INSERT INTO event_attendance_presences (event_id, user_id, role)
        VALUES (:event_id, :user_id, :role);",
    )?;
    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
        "role" => role,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn event_attendance_presence_remove(
    conn: &mut PooledConn,
    event_id: u64,
    user_id: u64,
    role: &str,
) -> Result<(), Error> {
    let stmt = conn.prep(
        "DELETE FROM event_attendance_presences
        WHERE event_id = :event_id AND user_id = :user_id AND role = :role;",
    )?;

    let params = params! {
        "event_id" => &event_id,
        "user_id" => &user_id,
        "role" => role,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
