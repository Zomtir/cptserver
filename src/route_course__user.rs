use rocket::serde::json::Json;

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::api::ApiError;
use crate::db::get_pool_conn;
use crate::session::UserSession;
use crate::common::{Course, Slot, Location, Branch, Access};

/*
 * ROUTES
 */

 #[rocket::get("/user/course_list")]
pub fn course_list(session: UserSession) -> Json<Vec<Course>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("
        SELECT c.course_id, c.course_key, c.title, c.active,
            b.branch_id, b.branch_key, b.title, c.threshold, COALESCE(skill.rank,0) as saneskill,
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
        AND c.threshold <= saneskill").unwrap();
    
    let params = params! { "user_id" => session.user.id};

    let map = |(course_id, course_key, course_title, active,
            branch_id, branch_key, branch_title, threshold,
            access_id, access_key, access_title): (u32, String, String, bool, u16, String, String, u8, u8, String, String)|
        Course {
            id: course_id, key: course_key, title: course_title, active,
            branch: Branch{id: branch_id, key: branch_key, title: branch_title}, threshold,
            access: Access{id: access_id, key: access_key, title: access_title}};
    
    let courses = conn.exec_map(&stmt,&params,&map).unwrap();
    return Json(courses);
}

#[rocket::get("/user/course_slot_list?<course_id>")]
pub fn course_slot_list(session: UserSession, course_id: u32) -> Json<Vec<Slot>> {
    // TODO check if course is public

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT slot_id, slot_key, s.title, l.location_id, l.location_key, l.title, s.begin, s.end, s.status
                          FROM slots s
                          JOIN locations l ON l.location_id = s.location_id
                          JOIN course_moderators m ON s.course_id = m.course_id
                          JOIN users u ON m.user_id = u.user_id
                          WHERE s.course_id = :course_id AND m.user_id = :user_id").unwrap();

    let params = params! {
        "course_id" => course_id,
        "user_id" => session.user.id,
    };

    let map = |(slot_id, slot_key, slot_title, location_id, location_key, location_title, begin, end, status): (u32, _, _, u32, _, _, _, _, String)|
        Slot {
            id: slot_id, key: slot_key, pwd: None, title: slot_title, begin, end, status: Some(status),
            location: Location {id: location_id, key: location_key, title: location_title},
            course_id: Some(course_id), owners: None};
    
    let slots = conn.exec_map(&stmt,&params,&map).unwrap();
    return Json(slots);
}
