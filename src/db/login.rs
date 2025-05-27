use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Right, User};
use crate::error::ErrorKind;

pub fn user_login(conn: &mut PooledConn, user_key: &str, salted_hash: &Vec<u8>) -> Result<User, ErrorKind> {
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, uc.sp_hash, uc.pepper, uc.salt, u.enabled, u.firstname, u.lastname, u.nickname
        FROM users u
        LEFT JOIN user_credentials uc ON uc.credential_id = u.credential
        WHERE u.user_key = :user_key;",
    )?;
    let params = params! {
        "user_key" => user_key,
    };

    // User is missing
    let mut row: mysql::Row = match conn.exec_first(&stmt, &params)? {
        None => return Err(ErrorKind::UserMissing),
        Some(row) => row,
    };

    // User is disabled
    if !row.take::<bool, &str>("enabled").unwrap() {
        return Err(ErrorKind::UserDisabled);
    }

    let user: User = User::from_info(
        row.take("user_id").unwrap(),
        row.take("user_key").unwrap(),
        row.take("firstname").unwrap(),
        row.take("lastname").unwrap(),
        row.take("nickname").unwrap(),
    );

    // User has no password configured
    let (pepper, salt): (Vec<u8>, Vec<u8>) = match (row.take("pepper").unwrap(), row.take("salt").unwrap()) {
        (Some(pepper), Some(salt)) => (pepper, salt),
        _ => return Err(ErrorKind::UserPasswordMissing),
    };
    let peppered_hash: Vec<u8> = crate::common::hash_sha256(&salted_hash, &pepper);

    println!(
        "User {} login attempt with salted hash {} (salt {}) resulting in peppered hash {} (pepper {})",
        user_key,
        hex::encode(&salted_hash),
        hex::encode(&salt),
        hex::encode(&peppered_hash),
        hex::encode(&pepper)
    );

    let peppered_hash_db: Vec<u8> = row.take("sp_hash").unwrap();

    if peppered_hash != peppered_hash_db {
        return Err(ErrorKind::UserLoginFail);
    };

    Ok(user)
}

pub fn user_right(conn: &mut PooledConn, user_id: u64) -> Result<Right, ErrorKind> {
    let stmt = conn.prep(
        "SELECT
            COALESCE(MAX(right_club_write),0) AS right_club_write,
            COALESCE(MAX(right_club_read),0) AS right_club_read,
            COALESCE(MAX(right_competence_write),0) AS right_competence_write,
            COALESCE(MAX(right_competence_read),0) AS right_competence_read,
            COALESCE(MAX(right_course_write),0) AS right_course_write,
            COALESCE(MAX(right_course_read),0) AS right_course_read,
            COALESCE(MAX(right_event_write),0) AS right_event_write,
            COALESCE(MAX(right_event_read),0) AS right_event_read,
            COALESCE(MAX(right_inventory_write),0) AS right_inventory_write,
            COALESCE(MAX(right_inventory_read),0) AS right_inventory_read,
            COALESCE(MAX(right_location_write),0) AS right_location_write,
            COALESCE(MAX(right_location_read),0) AS right_location_read,
            COALESCE(MAX(right_organisation_write),0) AS right_organisation_write,
            COALESCE(MAX(right_organisation_read),0) AS right_organisation_read,
            COALESCE(MAX(right_team_write),0) AS right_team_write,
            COALESCE(MAX(right_team_read),0) AS right_team_read,
            COALESCE(MAX(right_user_write),0) AS right_user_write,
            COALESCE(MAX(right_user_read),0) AS right_user_read
        FROM users u
        LEFT JOIN team_members ON (u.user_id = team_members.user_id)
        LEFT JOIN teams ON (team_members.team_id = teams.team_id)
        WHERE u.user_id = :user_id
        GROUP BY u.user_id;",
    )?;
    let params = params! {
        "user_id" => user_id,
    };

    let mut row: mysql::Row = match conn.exec_first(&stmt, &params)? {
        None => return Err(ErrorKind::UserMissing),
        Some(row) => row,
    };

    let right = Right {
        right_club_write: row.take("right_club_write").unwrap(),
        right_club_read: row.take("right_club_read").unwrap(),
        right_competence_write: row.take("right_competence_write").unwrap(),
        right_competence_read: row.take("right_competence_read").unwrap(),
        right_course_write: row.take("right_course_write").unwrap(),
        right_course_read: row.take("right_course_read").unwrap(),
        right_event_write: row.take("right_event_write").unwrap(),
        right_event_read: row.take("right_event_read").unwrap(),
        right_inventory_write: row.take("right_inventory_write").unwrap(),
        right_inventory_read: row.take("right_inventory_read").unwrap(),
        right_location_write: row.take("right_location_write").unwrap(),
        right_location_read: row.take("right_location_read").unwrap(),
        right_organisation_write: row.take("right_organisation_write").unwrap(),
        right_organisation_read: row.take("right_organisation_read").unwrap(),
        right_team_write: row.take("right_team_write").unwrap(),
        right_team_read: row.take("right_team_read").unwrap(),
        right_user_write: row.take("right_user_write").unwrap(),
        right_user_read: row.take("right_user_read").unwrap(),
    };

    Ok(right)
}

pub fn event_credential(conn: &mut PooledConn, event_id: u64) -> Result<(String, String), ErrorKind> {
    let stmt = conn.prep(
        "SELECT event_key, pwd
        FROM events
        WHERE event_id = :event_id;",
    )?;
    let params = params! {
        "event_id" => event_id,
    };

    let mut row: mysql::Row = match conn.exec_first(&stmt, &params)? {
        None => return Err(ErrorKind::EventMissing),
        Some(row) => row,
    };

    let event_key: String = row.take("event_key").unwrap();
    let event_pwd: String = row.take("pwd").unwrap();

    Ok((event_key, event_pwd))
}

pub fn event_login(conn: &mut PooledConn, event_key: &str) -> Result<(u64, String), ErrorKind> {
    let stmt = conn.prep(
        "SELECT event_id, pwd
        FROM events WHERE event_key = :event_key",
    )?;
    let params = params! {
        "event_key" => event_key,
    };

    let mut row: mysql::Row = match conn.exec_first(&stmt, &params)? {
        None => return Err(ErrorKind::EventMissing),
        Some(row) => row,
    };

    let event_id: u64 = row.take("event_id").unwrap();
    let event_pwd: String = row.take("pwd").unwrap();
    Ok((event_id, event_pwd))
}

pub fn course_current_event(
    conn: &mut PooledConn,
    course_key: &str,
    date_min: &chrono::NaiveDateTime,
    date_max: &chrono::NaiveDateTime,
) -> Result<(String, String), ErrorKind> {
    let stmt = conn.prep(
        "SELECT s.event_key, s.pwd
        FROM events s
        JOIN courses c ON c.course_id = s.course_id
        WHERE c.course_key = :course_key
        AND s.begin >= :date_min AND s.end <= :date_max
        AND c.active = TRUE",
    )?;
    let params = params! {
        "course_key" => course_key,
        "date_min" => date_min,
        "date_max" => date_max,
    };

    let mut row: mysql::Row = match conn.exec_first(&stmt, &params)? {
        None => return Err(ErrorKind::EventMissing),
        Some(row) => row,
    };

    let event_key: String = row.take("event_key").unwrap();
    let event_pwd: String = row.take("pwd").unwrap();
    Ok((event_key, event_pwd))
}

pub fn location_current_event(
    conn: &mut PooledConn,
    location_key: &str,
    date_min: &chrono::NaiveDateTime,
    date_max: &chrono::NaiveDateTime,
) -> Result<(String, String), ErrorKind> {
    let stmt = conn.prep(
        "SELECT s.event_key, s.pwd
        FROM events s
        JOIN locations l ON l.location_id = s.location_id
        WHERE l.location_key = :location_key
        AND s.begin >= :date_min AND s.end <= :date_max
        AND public = 1",
    )?;
    let params = params! {
        "location_key" => location_key,
        "date_min" => date_min,
        "date_max" => date_max,
    };

    let mut row: mysql::Row = match conn.exec_first(&stmt, &params) {
        Err(..) | Ok(None) => return Err(ErrorKind::EventMissing),
        Ok(Some(row)) => row,
    };
    let event_key: String = row.take("event_key").unwrap();
    let event_pwd: String = row.take("pwd").unwrap();
    Ok((event_key, event_pwd))
}
