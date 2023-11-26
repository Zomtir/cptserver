use mysql::prelude::Queryable;
use mysql::{params, PooledConn};

use crate::common::{Branch, Course, Team, User};
use crate::db::get_pool_conn;
use crate::error::Error;

/*
 * METHODS
 */

pub fn list_courses(mod_id: Option<i64>, active: Option<bool>, public: Option<bool>) -> Result<Vec<Course>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT c.course_id, c.course_key, c.title, c.active, c.public,
            b.branch_id, b.branch_key, b.title, c.threshold
        FROM courses c
        JOIN branches b ON c.branch_id = b.branch_id
        LEFT JOIN course_moderators m ON c.course_id = m.course_id
        WHERE (:mod_id IS NULL OR m.user_id = :mod_id)
        AND (:active IS NULL OR c.active = :active)
        AND (:public IS NULL OR c.public = :public)
        GROUP BY c.course_id",
    )?;
    // TODO the WHERE and GROUP BY clause can be removed, if the user moderator filter is deemed to be useless

    let params = params! {
        "mod_id" => mod_id,
        "active" => active,
        "public" => public,
    };

    let map =
        |(course_id, course_key, course_title, active, public, branch_id, branch_key, branch_title, threshold)| {
            Course {
                id: course_id,
                key: course_key,
                title: course_title,
                active,
                public,
                branch: Branch {
                    id: branch_id,
                    key: branch_key,
                    title: branch_title,
                },
                threshold,
            }
        };

    match conn.exec_map(&stmt, &params, &map)? {
        courses => Ok(courses),
    }
}

pub fn available_courses(user_id: i64) -> Result<Vec<Course>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT c.course_id, c.course_key, c.title, c.active, c.public,
            b.branch_id, b.branch_key, b.title, c.threshold
        FROM courses c
        JOIN branches b ON c.branch_id = b.branch_id
        LEFT JOIN
        (
            SELECT b.branch_id as branch_id, COALESCE(MAX(r.rank),0) as rank
            FROM rankings r
            JOIN branches b ON r.branch_id = b.branch_id
            WHERE r.user_id = :user_id
            GROUP BY r.branch_id
        ) AS skill ON c.branch_id = skill.branch_id
        WHERE c.active = TRUE
        AND c.threshold <= COALESCE(skill.rank,0)",
    )?;

    let params = params! {
        "user_id" => user_id,
    };

    let map =
        |(course_id, course_key, course_title, active, public, branch_id, branch_key, branch_title, threshold)| {
            Course {
                id: course_id,
                key: course_key,
                title: course_title,
                active,
                public,
                branch: Branch {
                    id: branch_id,
                    key: branch_key,
                    title: branch_title,
                },
                threshold,
            }
        };

    match conn.exec_map(&stmt, &params, &map)? {
        courses => Ok(courses),
    }
}

pub fn responsible_courses(user_id: i64) -> Result<Vec<Course>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT c.course_id, c.course_key, c.title, c.active, c.public,
            b.branch_id, b.branch_key, b.title, c.threshold
        FROM courses c
        JOIN branches b ON c.branch_id = b.branch_id
        JOIN course_moderators m ON c.course_id = m.course_id
        WHERE m.user_id = :user_id",
    )?;

    let params = params! {
        "user_id" => user_id,
    };

    let map =
        |(course_id, course_key, course_title, active, public, branch_id, branch_key, branch_title, threshold)| {
            Course {
                id: course_id,
                key: course_key,
                title: course_title,
                active,
                public,
                branch: Branch {
                    id: branch_id,
                    key: branch_key,
                    title: branch_title,
                },
                threshold,
            }
        };

    match conn.exec_map(&stmt, &params, &map)? {
        courses => Ok(courses),
    }
}

pub fn create_course(course: &Course) -> Result<u32, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO courses (course_key, title, active, public, branch_id, threshold)
        VALUES (:course_key, :title, :active, :public, :branch_id, :threshold)",
    )?;
    let params = params! {
        "course_key" => crate::common::random_string(10),
        "title" => &course.title,
        "active" => &course.active,
        "public" => &course.public,
        "branch_id" => &course.branch.id,
        "threshold" => &course.threshold,
    };

    conn.exec_drop(&stmt, &params)?;

    Ok(conn.last_insert_id() as u32)
}

pub fn list_course_moderators(course_id: i64) -> Result<Vec<User>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname
        FROM users u
        JOIN course_moderators m ON m.user_id = u.user_id
        WHERE m.course_id = :course_id",
    )?;

    let params = params! {
        "course_id" => course_id,
    };
    let map = |(user_id, user_key, firstname, lastname)| User::from_info(user_id, user_key, firstname, lastname);

    match conn.exec_map(&stmt, &params, &map)? {
        members => Ok(members),
    }
}

pub fn is_course_moderator(course_id: i64, user_id: i64) -> Result<bool, Error> {
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

pub fn add_course_moderator(course_id: i64, user_id: i64) -> Option<()> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO course_moderators (course_id, user_id)
                          SELECT :course_id, :user_id",
    );
    let params = params! {
        "course_id" => &course_id,
        "user_id" => &user_id,
    };

    match conn.exec_drop(&stmt.unwrap(), &params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn remove_course_moderator(course_id: i64, user_id: i64) -> Option<()> {
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

    match conn.exec_drop(&stmt, &params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn list_course_teaminvites(course_id: i64) -> Result<Vec<Team>, Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT t.team_id, t.name, t.description
        FROM course_teaminvites c
        LEFT JOIN teams t ON c.team_id = t.team_id
        WHERE course_id = :course_id;",
    )?;
    let params = params! {
        "course_id" => course_id,
    };
    let map = |(team_id, name, description)| Team {
        id: team_id,
        name,
        description,
        right: None,
    };

    let teams = conn.exec_map(&stmt, &params, &map)?;
    Ok(teams)
}

pub fn add_course_teaminvite(course_id: i64, team_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "INSERT INTO course_teaminvites (course_id, team_id)
        VALUES (:course_id, :team_id);",
    )?;
    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}

pub fn remove_course_teaminvite(course_id: i64, team_id: i64) -> Result<(), Error> {
    let mut conn: PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "DELETE FROM course_teaminvites
        WHERE course_id = :course_id AND team_id = :team_id;",
    )?;

    let params = params! {
        "course_id" => &course_id,
        "team_id" => &team_id,
    };

    conn.exec_drop(&stmt, &params)?;
    Ok(())
}
