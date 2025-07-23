pub mod attendance;
pub mod moderator;

use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Course, Event, Requirement, Skill, User};
use crate::error::ErrorKind;

pub fn course_list(
    conn: &mut PooledConn,
    mod_id: Option<u64>,
    active: Option<bool>,
    public: Option<bool>,
) -> Result<Vec<Course>, ErrorKind> {
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

pub fn course_available(conn: &mut PooledConn, user_id: u64) -> Result<Vec<Course>, ErrorKind> {
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

pub fn course_create(conn: &mut PooledConn, course: &Course) -> Result<u32, ErrorKind> {
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

pub fn course_edit(conn: &mut PooledConn, course_id: u32, course: &Course) -> Result<(), ErrorKind> {
    if course.key.is_empty() {
        return Err(ErrorKind::CourseKeyInvalid);
    }

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

pub fn course_delete(conn: &mut PooledConn, course_id: u32) -> Result<(), ErrorKind> {
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

/* REQUIREMENTS */

pub fn course_requirement_list(conn: &mut PooledConn, course_id: u32) -> Result<Vec<Requirement>, ErrorKind> {
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

pub fn course_requirement_add(
    conn: &mut PooledConn,
    course_id: u32,
    skill_id: u32,
    rank: u32,
) -> Result<(), ErrorKind> {
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

pub fn course_requirement_remove(conn: &mut PooledConn, requirement_id: u64) -> Result<(), ErrorKind> {
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

/* CLUB RELATED */

pub fn course_club_info(conn: &mut PooledConn, course_id: u64) -> Result<Option<u32>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT club_id
        FROM courses
        WHERE course_id = :course_id",
    )?;
    let params = params! {
        "course_id" => course_id,
    };

    match conn.exec_first::<Option<u32>, _, _>(&stmt, &params)? {
        None => Err(ErrorKind::CourseMissing),
        Some(club_id) => Ok(club_id),
    }
}

pub fn course_club_edit(conn: &mut PooledConn, course_id: u64, club_id: Option<u32>) -> Result<(), ErrorKind> {
    let stmt = conn.prep(
        "UPDATE courses
        SET club_id = :club_id
        WHERE course_id = :course_id",
    )?;

    let params = params! {
        "course_id" => &course_id,
        "club_id" => &club_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

/* STATISTICS */

pub fn course_statistic_class(
    conn: &mut PooledConn,
    course_id: u32,
) -> Result<Vec<(Event, u64, u64, u64, u64)>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT 
            events.event_id,
            events.event_key,
            events.title,
            events.begin,
            events.end,
            COUNT(DISTINCT CASE WHEN role = 'LEADER' THEN user_id END) AS leader_count,
            COUNT(DISTINCT CASE WHEN role = 'SUPPORTER' THEN user_id END) AS supporter_count,
            COUNT(DISTINCT CASE WHEN role = 'PARTICIPANT' THEN user_id END) AS participant_count,
            COUNT(DISTINCT CASE WHEN role = 'SPECTATOR' THEN user_id END) AS spectator_count
        FROM
            events
        LEFT JOIN
            event_attendance_presences ON events.event_id = event_attendance_presences.event_id
        WHERE
            events.course_id = :course_id
        GROUP BY
            events.event_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
    };

    let map = |(
        event_id,
        event_key,
        title,
        begin,
        end,
        leader_count,
        supporter_count,
        participant_count,
        spectator_count,
    )| {
        (
            Event::from_info(event_id, event_key, title, begin, end, None),
            leader_count,
            supporter_count,
            participant_count,
            spectator_count,
        )
    };

    let stats = conn.exec_map(&stmt, &params, &map)?;
    Ok(stats)
}

pub fn course_statistic_attendance(
    conn: &mut PooledConn,
    course_id: u32,
    role: String,
) -> Result<Vec<(User, u64)>, ErrorKind> {
    let stmt = conn.prep(
        "SELECT
            u.user_id,
            u.user_key,
            u.firstname,
            u.lastname,
            u.nickname,
            COUNT(p.event_id)
        FROM
            users u
        JOIN
            event_attendance_presences p ON u.user_id = p.user_id
        JOIN
            events ON p.event_id = events.event_id
        WHERE
            events.course_id = :course_id AND p.role = :role
        GROUP BY
            u.user_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
        "role" => &role,
    };

    let map = |(user_id, user_key, firstname, lastname, nickname, count)| {
        (User::from_info(user_id, user_key, firstname, lastname, nickname), count)
    };

    let stats = conn.exec_map(&stmt, &params, &map)?;
    Ok(stats)
}

pub fn course_statistic_attendance1(
    conn: &mut PooledConn,
    course_id: u32,
    user_id: u64,
    role: String,
) -> Result<Vec<Event>, ErrorKind> {
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
            event_attendance_presences p ON events.event_id = p.event_id
        WHERE
            events.course_id = :course_id AND p.user_id = :user_id AND role = :role;",
    )?;

    let params = params! {
        "course_id" => &course_id,
        "user_id" => &user_id,
        "role" => &role,
    };

    let map = Event::sql_map();

    let stats = conn.exec_map(&stmt, &params, &map)?;
    Ok(stats)
}
