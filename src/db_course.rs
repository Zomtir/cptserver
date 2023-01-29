use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::db::get_pool_conn;
use crate::common::{User, Access, Course, Branch};

/*
 * METHODS
 */

pub fn list_courses(mod_id: Option<i64>) -> Option<Vec<Course>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT c.course_id, c.course_key, c.title, c.active,
            b.branch_id, b.branch_key, b.title, c.threshold,
            a.access_id, a.access_key, a.title
        FROM courses c
        JOIN branches b ON c.branch_id = b.branch_id
        JOIN access a ON c.access_id = a.access_id
        LEFT JOIN course_moderators m ON c.course_id = m.course_id
        WHERE (:mod_id IS NULL OR m.user_id = :mod_id)
        GROUP BY c.course_id");
    // TODO the WHERE and GROUP BY clause can be removed, if the user filter is deemed to be useless
    // TODO add filter whether or not the course is active
        
    let params = params! {
        "mod_id" => mod_id,
    };

    let map = |(course_id, course_key, course_title, active,
            branch_id, branch_key, branch_title, threshold,
            access_id, access_key, access_title): (u32, String, String, bool, u16, String, String, u8, u8, String, String)|
        Course {
            id: course_id, key: course_key, title: course_title, active,
            branch: Branch{id: branch_id, key: branch_key, title: branch_title}, threshold,
            access: Access{id: access_id, key: access_key, title: access_title}};
    
    match conn.exec_map(&stmt.unwrap(),&params,&map) {
        Err(..) => None,
        Ok(courses) => Some(courses),
    }
}

pub fn available_courses(user_id: i64) -> Option<Vec<Course>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("
        SELECT c.course_id, c.course_key, c.title, c.active,
            b.branch_id, b.branch_key, b.title, c.threshold,
            a.access_id, a.access_key, a.title
        FROM courses c
        JOIN branches b ON c.branch_id = b.branch_id
        JOIN access a ON c.access_id = a.access_id
        LEFT JOIN
        (
            SELECT b.branch_id as branch_id, COALESCE(MAX(r.rank),0) as rank
            FROM rankings r
            JOIN branches b ON r.branch_id = b.branch_id
            WHERE r.user_id = :user_id
            GROUP BY r.branch_id
        ) AS skill ON c.branch_id = skill.branch_id
        WHERE c.active = TRUE
        AND c.threshold <= COALESCE(skill.rank,0)");
    
    let params = params! {
        "user_id" => user_id,
    };

    let map = |(course_id, course_key, course_title, active,
            branch_id, branch_key, branch_title, threshold,
            access_id, access_key, access_title): (u32, String, String, bool, u16, String, String, u8, u8, String, String)|
        Course {
            id: course_id, key: course_key, title: course_title, active,
            branch: Branch{id: branch_id, key: branch_key, title: branch_title}, threshold,
            access: Access{id: access_id, key: access_key, title: access_title}};
    
    match conn.exec_map(&stmt.unwrap(),&params,&map) {
        Err(..) => None,
        Ok(courses) => Some(courses),
    }
}

pub fn responsible_courses(user_id: i64) -> Option<Vec<Course>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("
        SELECT c.course_id, c.course_key, c.title, c.active,
            b.branch_id, b.branch_key, b.title, c.threshold,
            a.access_id, a.access_key, a.title
        FROM courses c
        JOIN branches b ON c.branch_id = b.branch_id
        JOIN access a ON c.access_id = a.access_id
        JOIN course_moderators m ON c.course_id = m.course_id
        WHERE m.user_id = :user_id");
    
    let params = params! {
        "user_id" => user_id,
    };

    let map = |(course_id, course_key, course_title, active,
            branch_id, branch_key, branch_title, threshold,
            access_id, access_key, access_title): (u32, String, String, bool, u16, String, String, u8, u8, String, String)|
        Course {
            id: course_id, key: course_key, title: course_title, active,
            branch: Branch{id: branch_id, key: branch_key, title: branch_title}, threshold,
            access: Access{id: access_id, key: access_key, title: access_title}
        };
    
    match conn.exec_map(&stmt.unwrap(),&params,&map) {
        Err(..) => None,
        Ok(courses) => Some(courses),
    }
}

pub fn create_course(course: &Course) -> Option<u32> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO courses (course_key, title, active, access_id, branch_id, threshold)
        VALUES (:course_key, :title, :active, :access_id, :branch_id, :threshold)");
    let params = params! {
        "course_key" => crate::common::random_string(10),
        "title" => &course.title,
        "active" => &course.active,
        "access_id" => &course.access.id,
        "branch_id" => &course.branch.id,
        "threshold" => &course.threshold,
    };

    match conn.exec_drop(&stmt.unwrap(),&params) {
        Err(..) => return None,
        Ok(..) => (),
    };

    crate::db::get_last_id(conn)
}

pub fn list_course_moderators(course_id: i64) -> Option<Vec<User>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT u.user_id, u.user_key, u.firstname, u.lastname
        FROM users u
        JOIN course_moderators m ON m.user_id = u.user_id
        WHERE m.course_id = :course_id");

    let params = params! {
        "course_id" => course_id,
    };
    let map = |(user_id, user_key, firstname, lastname)| {
        User::from_info(user_id, user_key, firstname, lastname)
    };

    match conn.exec_map(&stmt.unwrap(), &params, &map) {
        Err(..) => None,
        Ok(members) => Some(members),
    }
}

pub fn is_course_moderator(course_id: i64, user_id: i64) -> Option<bool> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep(
        "SELECT COUNT(1)
        FROM course_moderators
        WHERE course_id = :course_id AND user_id = :user_id");

    let params = params! {
        "course_id" => course_id,
        "user_id" => user_id,
    };

    match conn.exec_first::<u32,_,_>(&stmt.unwrap(), &params){
        Err(..) => return None,
        Ok(None) => return Some(false),
        Ok(Some(count)) => return Some(count == 1),
    };
}

pub fn add_course_moderator(course_id: i64, user_id: i64) -> Option<()> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("INSERT INTO course_moderators (course_id, user_id)
                          SELECT :course_id, :user_id");
    let params = params! {
        "course_id" => &course_id,
        "user_id" => &user_id,
    };

    match conn.exec_drop(&stmt.unwrap(),&params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}

pub fn remove_course_moderator(course_id: i64, user_id: i64) -> Option<()> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("DELETE e FROM course_moderators e
                          WHERE course_id = :course_id AND user_id = :user_id").unwrap();
    let params = params! {
        "course_id" => &course_id,
        "user_id" => &user_id,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => None,
        Ok(..) => Some(()),
    }
}
