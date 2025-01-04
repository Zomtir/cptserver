use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Affiliation, Event, User};
use crate::error::Error;

/* HOUSEKEEPING */

pub fn club_team_comparison(
    conn: &mut PooledConn,
    club_id: u32,
    team_id: u32,
    point_in_time: chrono::NaiveDate,
) -> Result<Vec<User>, Error> {
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM users u
        INNER JOIN (
            SELECT tm.user_id
            FROM team_members tm
            WHERE tm.team_id = :team_id
            EXCEPT
            SELECT t.user_id
            FROM terms t
            WHERE t.club_id = :club_id
            AND (:point_in_time BETWEEN COALESCE(t.term_begin, :point_in_time) AND COALESCE(t.term_end, :point_in_time))
        ) AS missing ON missing.user_id = u.user_id;",
    )?;

    let params = params! {
        "club_id" => club_id,
        "team_id" => team_id,
        "point_in_time" => point_in_time,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let nonmembers = conn.exec_map(&stmt, &params, &map)?;
    Ok(nonmembers)
}

// TODO I think the clause "WHERE :point_in_time BETWEEN et.effective_begin AND et.effective_end"
// disregards other terms and should be removed
pub fn club_member_leaderboard(
    conn: &mut PooledConn,
    club_id: u32,
    active: Option<bool>,
    point_in_time: chrono::NaiveDate,
) -> Result<Vec<(User, u32)>, Error> {
    let stmt = conn.prep(
        "WITH effective_terms AS (
            SELECT t.user_id,
                LEAST(COALESCE(t.term_end, :point_in_time), :point_in_time) AS effective_end,
                LEAST(COALESCE(t.term_begin, :point_in_time), :point_in_time) AS effective_begin
            FROM terms t
            WHERE t.club_id = :club_id
        )
        SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
               SUM(DATEDIFF(et.effective_end, et.effective_begin)) AS active_days
        FROM effective_terms et
        JOIN users u ON u.user_id = et.user_id
        WHERE :point_in_time BETWEEN et.effective_begin AND et.effective_end
        AND (:active IS NULL OR :active = u.active)
        GROUP BY u.user_id
        ORDER BY active_days DESC;",
    )?;

    let params = params! {
        "active" => &active,
        "club_id" => &club_id,
        "point_in_time" => point_in_time,
    };

    let map = |(user_id, user_key, firstname, lastname, nickname, active_days)| {
        (
            User::from_info(user_id, user_key, firstname, lastname, nickname),
            active_days,
        )
    };

    let leaderboard = conn.exec_map(&stmt, &params, &map)?;
    Ok(leaderboard)
}

pub fn club_member_organisation(
    conn: &mut PooledConn,
    club_id: u32,
    organisation_id: u64,
    active: Option<bool>,
    point_in_time: chrono::NaiveDate,
) -> Result<Vec<Affiliation>, Error> {
    let stmt = conn.prep(
        "WITH effective_terms AS (
            SELECT t.user_id,
                COALESCE(t.term_end, :point_in_time) AS effective_end,
                COALESCE(t.term_begin, :point_in_time) AS effective_begin
            FROM terms t
            WHERE t.club_id = :club_id
        )
        SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname,
            o.organisation_id, o.abbreviation AS organisation_abbreviation, o.name AS organisation_name,
            oa.member_identifier, oa.permission_solo_date, oa.permission_team_date, oa.residency_move_date
        FROM effective_terms et
        JOIN users u ON u.user_id = et.user_id
        LEFT JOIN organisation_affiliations oa ON oa.user_id = u.user_id AND oa.organisation_id = :organisation_id
        LEFT JOIN organisations o ON o.organisation_id = oa.organisation_id
        WHERE :point_in_time BETWEEN et.effective_begin AND et.effective_end
        AND (:active IS NULL OR :active = u.active)
        GROUP BY u.user_id;",
    )?;

    let params = params! {
        "club_id" => &club_id,
        "organisation_id" => organisation_id,
        "active" => &active,
        "point_in_time" => point_in_time,
    };

    let rows: Vec<mysql::Row> = conn.exec(&stmt, &params)?;

    let mut affiliations: Vec<Affiliation> = Vec::new();

    for row in rows {
        affiliations.push(crate::db::organisation::sql_affiliation(row));
    }

    Ok(affiliations)
}

pub fn club_statistic_user_leader(
    conn: &mut PooledConn,
    club_id: u32,
    leader_id: u64,
    time_window_begin: chrono::NaiveDateTime,
    time_window_end: chrono::NaiveDateTime,
) -> Result<Vec<Event>, Error> {
    let stmt = conn.prep(
        "SELECT
            events.event_id,
            events.event_key,
            events.title,
            events.begin,
            events.end,
            locations.location_id,
            locations.location_key,
            locations.name AS location_name,
            locations.description AS location_description
        FROM
            events
        JOIN
            locations ON locations.location_id = events.location_id
        JOIN
            event_leader_presences p ON events.event_id = p.event_id
        JOIN
            courses ON events.course_id = courses.course_id
        WHERE
            courses.club_id = :club_id AND p.user_id = :leader_id
        AND
            events.begin BETWEEN :time_window_begin AND :time_window_end;",
    )?;

    let params = params! {
        "club_id" => &club_id,
        "leader_id" => &leader_id,
        "time_window_begin" => &time_window_begin,
        "time_window_end" => &time_window_end,
    };

    let map = Event::sqlmap();

    let stats = conn.exec_map(&stmt, &params, &map)?;
    Ok(stats)
}

pub fn club_statistic_user_participant(
    conn: &mut PooledConn,
    club_id: u32,
    participant_id: u64,
    time_window_begin: chrono::NaiveDateTime,
    time_window_end: chrono::NaiveDateTime,
) -> Result<Vec<Event>, Error> {
    let stmt = conn.prep(
        "SELECT
            events.event_id,
            events.event_key,
            events.title,
            events.begin,
            events.end,
            locations.location_id,
            locations.location_key,
            locations.name AS location_name,
            locations.description AS location_description
        FROM
            events
        JOIN
            locations ON locations.location_id = events.location_id
        JOIN
            event_participant_presences p ON events.event_id = p.event_id
        JOIN
            courses ON events.course_id = courses.course_id
        WHERE
            courses.club_id = :club_id AND p.user_id = :participant_id
        AND
            events.begin BETWEEN :time_window_begin AND :time_window_end;",
    )?;

    let params = params! {
        "club_id" => &club_id,
        "participant_id" => &participant_id,
        "time_window_begin" => &time_window_begin,
        "time_window_end" => &time_window_end,
    };

    let map = Event::sqlmap();

    let stats = conn.exec_map(&stmt, &params, &map)?;
    Ok(stats)
}

pub fn club_statistic_user_supporter(
    conn: &mut PooledConn,
    club_id: u32,
    supporter_id: u64,
    time_window_begin: chrono::NaiveDateTime,
    time_window_end: chrono::NaiveDateTime,
) -> Result<Vec<Event>, Error> {
    let stmt = conn.prep(
        "SELECT
            events.event_id,
            events.event_key,
            events.title,
            events.begin,
            events.end,
            locations.location_id,
            locations.location_key,
            locations.name AS location_name,
            locations.description AS location_description
        FROM
            events
        JOIN
            locations ON locations.location_id = events.location_id
        JOIN
            event_supporter_presences p ON events.event_id = p.event_id
        JOIN
            courses ON events.course_id = courses.course_id
        WHERE
            courses.club_id = :club_id AND p.user_id = :supporter_id
        AND
            events.begin BETWEEN :time_window_begin AND :time_window_end;",
    )?;

    let params = params! {
        "club_id" => &club_id,
        "supporter_id" => &supporter_id,
        "time_window_begin" => &time_window_begin,
        "time_window_end" => &time_window_end,
    };

    let map = Event::sqlmap();

    let stats = conn.exec_map(&stmt, &params, &map)?;
    Ok(stats)
}
