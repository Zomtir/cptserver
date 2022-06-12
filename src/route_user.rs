use rocket_contrib::json::Json;
use rocket::http::Status;
use crate::api::ApiError;

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::session::{POOL, UserSession, User, Course, Slot, Ranking, Member, Location, Branch, Access,
    random_string, random_bytes, verify_password, hash_sha256};

/*
 * ROUTES
 */

#[get("/user_info")]
pub fn user_info(session: UserSession) -> Json<User> {
    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("SELECT user_id, user_key, firstname, lastname, enabled FROM users
                          WHERE user_id = :user_id").unwrap();

    let params = mysql::params! { "user_id" => session.user.id };
    let map = |(id, key, firstname, lastname, enabled)| {
        User { id, key, pwd: None, firstname, lastname, enabled,
            admin_users: session.user.admin_users,
            admin_rankings: session.user.admin_rankings,
            admin_reservations: session.user.admin_reservations,
            admin_courses: session.user.admin_courses,
        }
    };

    let mut users = conn.exec_map(&stmt, &params, &map).unwrap();
    return Json(users.remove(0));
}

#[post("/user_password", format = "text/plain", data = "<password>")]
pub fn user_password(session: UserSession, password: String) {
    let bpassword : Vec<u8> = match verify_password(&password){
        Some(bpassword) => bpassword,
        None => return,
    };
    
    let pepper : Vec<u8> = random_bytes(16);
    let shapassword : Vec<u8> = hash_sha256(&bpassword, &pepper);

    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("UPDATE users SET pwd = :pwd, pepper = :pepper WHERE user_id = :user_id").unwrap();

    conn.exec::<String,_,_>(
        &stmt,
        mysql::params! {
            "user_id" => &session.user.id,
            "pwd" => &shapassword,
            "pepper" => &pepper,
        },
    ).unwrap();
}

#[get("/user_info_rankings")]
pub fn user_info_rankings(session: UserSession) -> Json<Vec<Ranking>> {
    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("SELECT r.ranking_id, b.branch_id, b.branch_key, b.title, r.rank, r.date, j.user_id, j.user_key, j.firstname, j.lastname
                          FROM rankings r
                          JOIN branches b ON (r.branch_id = b.branch_id)
                          JOIN users j ON (r.judge_id = j.user_id)
                          WHERE r.user_id = :user_id").unwrap();

    let map = |(ranking_id,
        branch_id, branch_key, branch_title,
        rank, date,
        judge_id, judge_key, judge_firstname, judge_lastname)
      : (u32,
        u16, String, String,
        u8, _,
        u32, String, String, String)|
      Ranking {id: ranking_id,
        user: Member::from_user(&session.user),
        branch: Branch{id: branch_id, key: branch_key, title: branch_title},
        rank, date,
        judge: Member{id: judge_id, key: judge_key, firstname: judge_firstname, lastname: judge_lastname}
      };

    let rankings = conn.exec_map(
        &stmt,
        mysql::params! { "user_id" => session.user.id },
        &map,
    ).unwrap();

    return Json(rankings);
}

// TODO only active members
// TODO restrict to admins (ranking and moderation at least)?
#[get("/user_member_list")]
pub fn user_member_list(_session: UserSession) -> Json<Vec<Member>> {
    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("SELECT user_id, user_key, firstname, lastname FROM users").unwrap();

    let members = conn.exec_map(
        &stmt,
        mysql::params::Params::Empty,
        |(user_id, user_key, firstname, lastname)| {
            Member{id: user_id, key: user_key, firstname, lastname}
        },
    ).unwrap();

    return Json(members);
}

#[get("/user_course_list")]
pub fn user_course_list(session: UserSession) -> Json<Vec<Course>> {
    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("SELECT c.course_id, c.course_key, c.title, c.active,
                            b.branch_id, b.branch_key, b.title, c.threshold,
                            a.access_id, a.access_key, a.title
                          FROM courses c
                          JOIN branches b ON c.branch_id = b.branch_id
                          JOIN access a ON c.access_id = a.access_id
                          JOIN course_moderators m ON c.course_id = m.course_id
                          WHERE m.user_id = :user_id").unwrap();
    
    let params = mysql::params! { "user_id" => session.user.id};

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

#[get("/indi_slot_list?<status>")]
pub fn indi_slot_list(session: UserSession, status: String) -> Result<Json<Vec<Slot>>,Status> {
    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("SELECT slot_id, slot_key, s.title, l.location_id, l.location_key, l.title, s.begin, s.end, s.status
                          FROM slots s
                          JOIN locations l ON l.location_id = s.location_id
                          WHERE user_id = :user_id AND status = :status").unwrap();

    let params = mysql::params! {
        "user_id" => session.user.id,
        "status" => &status,
    };
    let map = |(slot_id, slot_key, slot_title, location_id, location_key, location_title, begin, end, status): (u32, _, _, u32, _, _, _, _, String)| 
        Slot {
            id: slot_id, key: slot_key, title: slot_title, pwd: None, begin, end, status: Some(status),
            location: Location {id: location_id, key: location_key, title: location_title},
            course_id: None, user_id: Some(session.user.id)};
    
    match conn.exec_map(&stmt,&params,&map) {
        Err(..) => return Err(Status::Conflict),
        Ok(slots) => return Ok(Json(slots)),
    };
}

#[get("/course_slot_list?<course_id>")]
pub fn course_slot_list(session: UserSession, course_id: u32) -> Json<Vec<Slot>> {
    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("SELECT slot_id, slot_key, s.title, l.location_id, l.location_key, l.title, s.begin, s.end, s.status
                          FROM slots s
                          JOIN locations l ON l.location_id = s.location_id
                          JOIN course_moderators m ON s.course_id = m.course_id
                          JOIN users u ON m.user_id = u.user_id
                          WHERE s.course_id = :course_id AND m.user_id = :user_id").unwrap();

    let params = mysql::params! {
        "course_id" => course_id,
        "user_id" => session.user.id,
    };

    let map = |(slot_id, slot_key, slot_title, location_id, location_key, location_title, begin, end, status): (u32, _, _, u32, _, _, _, _, String)|
        Slot {
            id: slot_id, key: slot_key, pwd: None, title: slot_title, begin, end, status: Some(status),
            location: Location {id: location_id, key: location_key, title: location_title},
            course_id: Some(course_id), user_id: None};
    
    let slots = conn.exec_map(&stmt,&params,&map).unwrap();
    return Json(slots);
}

#[post("/indi_slot_create", format = "application/json", data = "<slot>")]
pub fn indi_slot_create(session: UserSession, mut slot: Json<Slot>) -> Result<String, Status> {
    crate::session::round_slot_window(&mut slot);

    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("INSERT INTO slots (slot_key, pwd, title, location_id, begin, end, status, user_id)
                          VALUES (:slot_key, :pwd, :title, :location_id, :begin, :end, :status, :user_id)").unwrap();

    let params = mysql::params! {
        "slot_key" => random_string(8),
        "pwd" => random_string(8),
        "title" => &slot.title,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
        "status" => "DRAFT",
        "user_id" => &session.user.id,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => return Err(Status::InternalServerError),
        Ok(..) => (),
    };
    
    let stmt_id = conn.prep("SELECT LAST_INSERT_ID()").unwrap();

    match conn.exec_first::<u32,_,_>(&stmt_id,params::Params::Empty) {
        Err(..) | Ok(None) => Err(Status::InternalServerError),
        Ok(Some(slot_id)) => Ok(slot_id.to_string()),
    }
}

#[post("/course_slot_create", format = "application/json", data = "<slot>")]
pub fn course_slot_create(session: UserSession, mut slot: Json<Slot>) -> Option<String> {
    crate::session::round_slot_window(&mut slot);

    if !crate::session::is_slot_valid(&mut slot) {return None;}

    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("INSERT INTO slots (slot_key, pwd, title, status, autologin, location_id, begin, end, course_id)
                          SELECT :slot_key, :pwd, :title, :status, :autologin, :location_id, :begin, :end, m.course_id
                          FROM course_moderators m
                          WHERE m.course_id = :course_id AND m.user_id = :user_id").unwrap();

    let params = mysql::params! {
        "slot_key" => random_string(8),
        "pwd" => random_string(8),
        "title" => &slot.title,
        "status" => "OCCURRING",
        "autologin" => false,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
        "course_id" => &slot.course_id,
        "user_id" => &session.user.id,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => return None,
        Ok(..) => (),
    };

    let stmt_id = conn.prep("SELECT LAST_INSERT_ID()").unwrap();

    let result_id = conn.exec_first::<u32,_,_>(
        &stmt_id,
        params::Params::Empty,
    );

    match result_id {
        Err(..) | Ok(None) => None,
        Ok(Some(slot_id)) => Some(slot_id.to_string()),
    }
}


// TODO, check times again... overall share more code with slot accept and slot_create
// TODO, allow inviting member for draft
// TODO, allow inviting groups for draft
#[post("/indi_slot_edit", format = "application/json", data = "<slot>")]
pub fn indi_slot_edit(session: UserSession, mut slot: Json<Slot>) {
    crate::session::round_slot_window(&mut slot);

    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("UPDATE slots SET
        title = :title,
        location_id = :location_id,
        begin = :begin,
        end = :end,
        status = 'DRAFT'
        WHERE slot_id = :slot_id AND user_id = :user_id
        AND (status = 'DRAFT' OR status = 'REJECTED' OR status = 'CANCELED')").unwrap();

    let params = mysql::params! {
        "slot_id" => &slot.id,
        "title" => &slot.title,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
        "user_id" => &session.user.id,
    };

    conn.exec::<String,_,_>(&stmt,&params).unwrap();

    if slot.pwd.is_none() || slot.pwd.as_ref().unwrap().len() < 8 {return};

    let stmt_pwd = conn.prep("UPDATE slots SET pwd = :pwd WHERE slot_id = :slot_id AND user_id = :user_id").unwrap();
    let params_pwd = mysql::params! {
        "slot_id" => &slot.id,
        "pwd" => &slot.pwd.as_ref().unwrap(),
        "user_id" => &session.user.id,
    };

    conn.exec::<String,_,_>(&stmt_pwd, &params_pwd).unwrap();
}

// TODO round slot times
#[post("/course_slot_edit", format = "application/json", data = "<slot>")]
pub fn course_slot_edit(session: UserSession, slot: Json<Slot>) -> Status {
    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("UPDATE slots s, course_moderators m SET
        s.title = :title,
        s.location_id = :location_id,
        s.begin = :begin,
        s.end = :end,
        s.status = 'OCCURRING'
        WHERE (s.course_id = m.course_id) AND s.slot_id = :slot_id AND s.course_id = :course_id AND m.user_id = :user_id").unwrap();

    let params = mysql::params! {
        "slot_id" => &slot.id,
        "title" => &slot.title,
        "location_id" => &slot.location.id,
        "begin" => &slot.begin,
        "end" => &slot.end,
        "course_id" => &slot.course_id,
        "user_id" => &session.user.id,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => return Status::Conflict,
        Ok(..) => (),
    };

    if slot.pwd.is_none() || slot.pwd.as_ref().unwrap().len() < 8 {return Status::Conflict};

    let stmt_pwd = conn.prep("UPDATE slots s, course_moderators m SET s.pwd = :pwd
                              WHERE (s.user_id = m.user_id)
                              AND s.slot_id = :slot_id
                              AND s.course_id = :course_id
                              AND m.user_id = :user_id").unwrap();

    let params_pwd = mysql::params! {
        "slot_id" => &slot.id,
        "pwd" => &slot.pwd.as_ref().unwrap(),
        "course_id" => &slot.course_id,
        "user_id" => &session.user.id,
    };

    match conn.exec::<String,_,_>(&stmt_pwd,&params_pwd) {
        Err(..) => Status::Conflict,
        Ok(..) => Status::Ok,
    }
}

#[head("/indi_slot_delete?<slot_id>")]
pub fn indi_slot_delete(session: UserSession, slot_id: u32) -> Status {
    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("DELETE s FROM slots s
                          WHERE slot_id = :slot_id AND user_id = :user_id").unwrap();
    let params = mysql::params! {
        "slot_id" => &slot_id,
        "user_id" => &session.user.id,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => Status::Conflict,
        Ok(..) => Status::Ok,
    }
}

#[head("/course_slot_delete?<slot_id>")]
pub fn course_slot_delete(session: UserSession, slot_id: u32) -> Status {
    let mut conn : PooledConn = POOL.clone().get_conn().unwrap();
    let stmt = conn.prep("DELETE s FROM slots s
                          JOIN course_moderators m ON s.course_id = m.course_id
                          WHERE s.slot_id = :slot_id AND m.user_id = :user_id").unwrap();
    let params = mysql::params! {
        "slot_id" => &slot_id,
        "user_id" => &session.user.id,
    };

    match conn.exec::<String,_,_>(&stmt,&params) {
        Err(..) => Status::Conflict,
        Ok(..) => Status::Ok,
    }
}


#[head("/indi_slot_submit?<slot_id>")]
pub fn indi_slot_submit(session: UserSession, slot_id: u32) -> Result<Status,ApiError> {
    // Perhaps lock the DB during checking and modifying the slot status

    let slot : Slot = match crate::session::get_slot_info(&slot_id){
        None => return Err(ApiError::SLOT_NO_ENTRY),
        Some(slot) => slot,
    };

    // Check that user is responsible for this slot
    if slot.id != session.user.id {
        return Err(ApiError::RIGHT_CONFLICT);
    }

    // The check is here intentional to be able to return early although it is also checked during is_slot_free
    if !crate::session::is_slot_valid(&slot) {
        return Err(ApiError::SLOT_BAD_TIME);
    }

    // Perhaps just leave the slot as draft if the time is not free
    let (status_update, response) = match crate::session::is_slot_free(&slot) {
        None => return Err(ApiError::DB_CONFLICT),
        Some(false) => ("REJECTED", Err(ApiError::SLOT_OVERLAP_TIME)),
        Some(true) => match crate::config::CONFIG_RESERVATION_AUTO_ACCEPT {
            false => ("PENDING", Ok(Status::Ok)),
            true => ("OCCURRING", Ok(Status::Ok)),
        },
    };
    
    match crate::session::set_slot_status(slot.id, "PENDING", status_update) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => response,
    }
}

// TODO check that user is allowed to edit this slot
#[head("/indi_slot_withdraw?<slot_id>")]
pub fn indi_slot_withdraw(_session: UserSession, slot_id: u32) -> Status {
    match crate::session::set_slot_status(slot_id, "PENDING", "DRAFT") {
        None => Status::InternalServerError,
        Some(..) => Status::Ok,
    }
}

// TODO check that user is allowed to edit this slot
#[head("/indi_slot_cancel?<slot_id>")]
pub fn indi_slot_cancel(_session: UserSession, slot_id: u32) -> Status {
    match crate::session::set_slot_status(slot_id, "OCCURRING", "CANCELED") {
        None => Status::InternalServerError,
        Some(..) => Status::Ok,
    }
}

// TODO check that user is allowed to edit this slot
#[head("/indi_slot_recycle?<slot_id>")]
pub fn indi_slot_recycle(_session: UserSession, slot_id: u32) -> Status {
    match crate::session::set_slot_status(slot_id, "REJECTED", "DRAFT") {
        None => Status::InternalServerError,
        Some(..) => Status::Ok,
    }
}
