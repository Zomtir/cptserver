use chrono::NaiveDateTime;
use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Course, Requirement, Skill, Team, User};
use crate::db::get_pool_conn;
use crate::error::Error;

/*
 * METHODS
 */

pub fn course_list(mod_id: Option<u64>, active: Option<bool>, public: Option<bool>) -> Result<Vec<Course>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT c.course_id, c.course_key, c.title, c.active, c.public
        FROM courses c
        LEFT JOIN course_moderators m ON c.course_id = m.course_id
        WHERE (:mod_id IS NULL OR m.user_id = :mod_id)
        AND (:active IS NULL OR c.active = :active)
        AND (:public IS NULL OR c.public = :public)
        GROUP BY c.course_id",
    )?;

    let params = params! {
        "mod_id" => mod_id,
        "active" => active,
        "public" => public,
    };

    let map = |(course_id, course_key, course_title, active, public)| Course {
        id: course_id,
        key: course_key,
        title: course_title,
        active,
        public,
    };

    let courses = conn.exec_map(&stmt, &params, &map)?;
    Ok(courses)
}

pub fn course_available(user_id: u64) -> Result<Vec<Course>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT DISTINCT c.course_id, c.course_key, c.title, c.active, c.public
        FROM courses c
        INNER JOIN course_requirements cr ON c.course_id = cr.course_id
        LEFT JOIN user_competences uc ON uc.skill_id = cr.skill_id AND uc.rank >= cr.rank AND uc.user_id = :user_id
        WHERE uc.user_id IS NOT NULL;",
    )?;

    let params = params! {
        "user_id" => user_id,
    };

    let map = |(course_id, course_key, course_title, active, public)| Course {
        id: course_id,
        key: course_key,
        title: course_title,
        active,
        public,
    };

    let courses = conn.exec_map(&stmt, &params, &map)?;
    Ok(courses)
}

pub fn course_create(course: &Course) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO courses (course_key, title, active, public)
        VALUES (:course_key, :title, :active, :public)",
    )?;
    let params = params! {
        "course_key" => crate::common::random_string(10),
        "title" => &course.title,
        "active" => &course.active,
        "public" => &course.public,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn course_edit(course_id: u64, course: &Course) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "UPDATE courses SET
            course_key = :course_key,
            title = :title,
            active = :active,
            public = :public
            WHERE course_id = :course_id",
    )?;

    let params = params! {
        "course_id" => &course_id,
        "course_key" => &course.key,
        "title" => &course.title,
        "active" => &course.active,
        "public" => &course.public,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(())
}

pub fn course_delete(course_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE c FROM courses c
        WHERE c.course_id = :course_id",
    )?;

    let params = params! {
        "course_id" => &course_id,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(())
}

pub fn course_moderator_list(course_id: u64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname, u.nickname
        FROM users u
        JOIN course_moderators m ON m.user_id = u.user_id
        WHERE m.course_id = :course_id",
    )?;

    let params = params! {
        "course_id" => course_id,
    };
    let map = |(user_id, user_key, firstname, lastname, nickname)| {
        User::from_info(user_id, user_key, firstname, lastname, nickname)
    };

    let members = conn.exec_map(&stmt, &params, &map)?;
    Ok(members)
}

pub fn course_moderator_true(course_id: u64, user_id: u64) -> Result<bool, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM course_moderators
        WHERE course_id = :course_id AND user_id = :user_id",
    )?;

    let params = params! {
        "course_id" => course_id,
        "user_id" => user_id,
    };

    match conn.exec_first::<u32, _, _>(&stmt, &params)? {
        None => Ok(false),
        Some(count) => Ok(count == 1),
    }
}

pub fn course_moderator_add(course_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO course_moderators (course_id, user_id)
                          SELECT :course_id, :user_id",
    )?;
    let params = params! {
        "course_id" => &course_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn course_moderator_remove(course_id: u64, user_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn
        .prep(
            "DELETE e FROM course_moderators e
            WHERE course_id = :course_id AND user_id = :user_id",
        )
        .unwrap();
    let params = params! {
        "course_id" => &course_id,
        "user_id" => &user_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* OWNER SUMMONS */

pub fn course_owner_summon_list(course_id: u64) -> Result<Vec<Team>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT t.team_id, t.key, t.name, t.description
        FROM course_owner_summons c
        LEFT JOIN teams t ON c.team_id = t.team_id
        WHERE course_id = :course_id;",
    )?;
    let params = params! {
        "course_id" => course_id,
    };
    let map = |(team_id, team_key, name, description)| Team {
        id: team_id,
        key: team_key,
        name,
        description,
        right: None,
    };

    let teams = conn.exec_map(&stmt, &params, &map)?;
    Ok(teams)
}

pub fn course_owner_summon_add(course_id: u64, team_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO course_owner_summons (course_id, team_id)
        VALUES (:course_id, :team_id);",
    )?;
    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn course_owner_summon_remove(course_id: u64, team_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM course_owner_summons
        WHERE course_id = :course_id AND team_id = :team_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* OWNER UNSUMMONS */

pub fn course_owner_unsummon_list(course_id: u64) -> Result<Vec<Team>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT t.team_id, t.key, t.name, t.description
        FROM course_owner_unsummons c
        LEFT JOIN teams t ON c.team_id = t.team_id
        WHERE course_id = :course_id;",
    )?;
    let params = params! {
        "course_id" => course_id,
    };
    let map = |(team_id, team_key, name, description)| Team {
        id: team_id,
        key: team_key,
        name,
        description,
        right: None,
    };

    let teams = conn.exec_map(&stmt, &params, &map)?;
    Ok(teams)
}

pub fn course_owner_unsummon_add(course_id: u64, team_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO course_owner_unsummons (course_id, team_id)
        VALUES (:course_id, :team_id);",
    )?;
    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn course_owner_unsummon_remove(course_id: u64, team_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM course_owner_unsummons
        WHERE course_id = :course_id AND team_id = :team_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* PARTICIPANT SUMMONS */

pub fn course_participant_summon_list(course_id: u64) -> Result<Vec<Team>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT t.team_id, t.key, t.name, t.description
        FROM course_participant_summons c
        LEFT JOIN teams t ON c.team_id = t.team_id
        WHERE course_id = :course_id;",
    )?;
    let params = params! {
        "course_id" => course_id,
    };
    let map = |(team_id, team_key, name, description)| Team {
        id: team_id,
        key: team_key,
        name,
        description,
        right: None,
    };

    let teams = conn.exec_map(&stmt, &params, &map)?;
    Ok(teams)
}

pub fn course_participant_summon_add(course_id: u64, team_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO course_participant_summons (course_id, team_id)
        VALUES (:course_id, :team_id);",
    )?;
    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn course_participant_summon_remove(course_id: u64, team_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM course_participant_summons
        WHERE course_id = :course_id AND team_id = :team_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* PARTICIPANT UNSUMMONS */

pub fn course_participant_unsummon_list(course_id: u64) -> Result<Vec<Team>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT t.team_id, t.key, t.name, t.description
        FROM course_participant_unsummons c
        LEFT JOIN teams t ON c.team_id = t.team_id
        WHERE course_id = :course_id;",
    )?;
    let params = params! {
        "course_id" => course_id,
    };
    let map = |(team_id, team_key, name, description)| Team {
        id: team_id,
        key: team_key,
        name,
        description,
        right: None,
    };

    let teams = conn.exec_map(&stmt, &params, &map)?;
    Ok(teams)
}

pub fn course_participant_unsummon_add(course_id: u64, team_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO course_participant_unsummons (course_id, team_id)
        VALUES (:course_id, :team_id);",
    )?;
    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn course_participant_unsummon_remove(course_id: u64, team_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM course_participant_unsummons
        WHERE course_id = :course_id AND team_id = :team_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* REQUIREMENTS */

pub fn course_requirement_list(course_id: u64) -> Result<Vec<Requirement>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT r.requirement_id,
            c.course_id, c.course_key, c.title, c.active, c.public,
            s.skill_id, s.skill_key, s.title, s.min, s.max,
            r.rank
            FROM course_requirements r
        JOIN courses c ON c.course_id = r.course_id
        JOIN skills s ON s.skill_id = r.skill_id
        WHERE c.course_id = :course_id;",
    )?;

    let params = params! {
        "course_id" => course_id,
    };
    let map = |(
        requirement_id,
        course_id,
        course_key,
        course_title,
        course_active,
        course_public,
        skill_id,
        skill_key,
        skill_title,
        skill_min,
        skill_max,
        rank,
    )| Requirement {
        id: requirement_id,
        course: Course {
            id: course_id,
            key: course_key,
            title: course_title,
            active: course_active,
            public: course_public,
        },
        skill: Skill {
            id: skill_id,
            key: skill_key,
            title: skill_title,
            min: skill_min,
            max: skill_max,
        },
        rank,
    };

    let reqs = conn.exec_map(&stmt, &params, &map)?;
    Ok(reqs)
}

pub fn course_requirement_add(course_id: u64, skill_id: u32, rank: u32) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO course_requirements (course_id, skill_id, rank)
        SELECT :course_id, :skill_id, :rank;",
    )?;
    let params = params! {
        "course_id" => &course_id,
        "skill_id" => &skill_id,
        "rank" => &rank,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn course_requirement_remove(requirement_id: u64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE r FROM course_requirements r
        WHERE r.requirement_id = :requirement_id;",
    )?;
    let params = params! {
        "requirement_id" => &requirement_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* STATISTICS */

pub fn course_statistic_class(
    course_id: u64,
) -> Result<Vec<(u64, String, NaiveDateTime, NaiveDateTime, u64, u64)>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT 
            events.event_id,
            events.title,
            events.begin,
            events.end,
            COUNT(DISTINCT event_participants.user_id) AS participant_count,
            COUNT(DISTINCT event_owners.user_id) AS owner_count
        FROM
            events
        LEFT JOIN
            event_participants ON events.event_id = event_participants.event_id
        LEFT JOIN
            event_owners ON events.event_id = event_owners.event_id
        WHERE
            events.course_id = :course_id
        GROUP BY
            events.event_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
    };

    let map = |(course_id, course_name, begin, end, participants, owners)| {
        (course_id, course_name, begin, end, participants, owners)
    };

    let stats = conn.exec_map(&stmt, &params, &map)?;
    Ok(stats)
}

pub fn course_statistic_participant(course_id: u64) -> Result<Vec<(u64, String, String, u64)>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            u.user_id,
            u.firstname,
            u.lastname,
            COUNT(p.event_id)
        FROM
            users u
        JOIN
            event_participants p ON u.user_id = p.user_id
        JOIN
            events ON p.event_id = events.event_id
        WHERE
            events.course_id = :course_id
        GROUP BY
            u.user_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
    };

    let map = |(user_id, firstname, lastname, count)| (user_id, firstname, lastname, count);

    let stats = conn.exec_map(&stmt, &params, &map)?;
    Ok(stats)
}

pub fn course_statistic_participant1(
    course_id: u64,
    participant_id: u64,
) -> Result<Vec<(u64, String, NaiveDateTime, NaiveDateTime)>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            events.event_id,
            events.title,
            events.begin,
            events.end
        FROM
            events
        JOIN
            event_participants p ON events.event_id = p.event_id
        WHERE
            events.course_id = :course_id AND p.user_id = :participant_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
        "participant_id" => &participant_id,
    };

    let map = |(event_id, title, begin, end)| (event_id, title, begin, end);

    let stats = conn.exec_map(&stmt, &params, &map)?;
    Ok(stats)
}

pub fn course_statistic_owner(course_id: u64) -> Result<Vec<(u64, String, String, u64)>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            u.user_id,
            u.firstname,
            u.lastname,
            COUNT(p.event_id)
        FROM
            users u
        JOIN
            event_owners p ON u.user_id = p.user_id
        JOIN
            events ON p.event_id = events.event_id
        WHERE
            events.course_id = :course_id
        GROUP BY
            u.user_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
    };

    let map = |(user_id, firstname, lastname, count)| (user_id, firstname, lastname, count);

    let stats = conn.exec_map(&stmt, &params, &map)?;
    Ok(stats)
}

pub fn course_statistic_owner1(
    course_id: u64,
    owner_id: u64,
) -> Result<Vec<(u64, String, NaiveDateTime, NaiveDateTime)>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT
            events.event_id,
            events.title,
            events.begin,
            events.end
        FROM
            events
        JOIN
            event_owners p ON events.event_id = p.event_id
        WHERE
            events.course_id = :course_id AND p.user_id = :owner_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
        "owner_id" => &owner_id,
    };

    let map = |(event_id, title, begin, end)| (event_id, title, begin, end);

    let stats = conn.exec_map(&stmt, &params, &map)?;
    Ok(stats)
}
