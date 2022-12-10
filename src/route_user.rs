use rocket::http::Status;
use rocket::serde::json::Json;

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::api::ApiError;
use crate::db::get_pool_conn;
use crate::session::UserSession;
use crate::common::{User, Ranking, Member, Branch};
use crate::common::{random_bytes};

/*
 * ROUTES
 */

#[rocket::get("/user_info")]
pub fn user_info(session: UserSession) -> Json<User> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT user_id, user_key, firstname, lastname, enabled FROM users
                          WHERE user_id = :user_id").unwrap();

    let params = params! { "user_id" => session.user.id };
    let map = |(id, key, firstname, lastname, enabled)| {
        User { id, key, pwd: None, firstname, lastname, enabled,
            admin_courses: session.user.admin_courses,
            admin_rankings: session.user.admin_rankings,
            admin_reservations: session.user.admin_reservations,
            admin_teams: session.user.admin_teams,
            admin_users: session.user.admin_users,
        }
    };

    let mut users = conn.exec_map(&stmt, &params, &map).unwrap();
    return Json(users.remove(0));
}

#[rocket::post("/user_password", format = "text/plain", data = "<password>")]
pub fn user_password(session: UserSession, password: String) -> Result<Status, ApiError> {
    let bpassword : Vec<u8> = match crate::common::verify_password(&password){
        Some(bpassword) => bpassword,
        None => return Err(ApiError::USER_BAD_PASSWORD),
    };
    
    let pepper : Vec<u8> = random_bytes(16);
    let shapassword : Vec<u8> = crate::common::hash_sha256(&bpassword, &pepper);

    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("UPDATE users SET pwd = :pwd, pepper = :pepper WHERE user_id = :user_id").unwrap();
    let params = params! {
        "user_id" => &session.user.id,
        "pwd" => &shapassword,
        "pepper" => &pepper,
    };

    match conn.exec_drop(&stmt,&params) {
        Err(..) => Err(ApiError::DB_CONFLICT),
        Ok(..) => Ok(Status::Ok),
    }
}

#[rocket::get("/user_info_rankings")]
pub fn user_info_rankings(session: UserSession) -> Json<Vec<Ranking>> {
    let mut conn : PooledConn = get_pool_conn();
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
        params! { "user_id" => session.user.id },
        &map,
    ).unwrap();

    return Json(rankings);
}

// TODO only active members
#[rocket::get("/user_member_list")]
pub fn user_member_list(_session: UserSession) -> Json<Vec<Member>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT user_id, user_key, firstname, lastname FROM users").unwrap();

    let members = conn.exec_map(
        &stmt,
        params::Params::Empty,
        |(user_id, user_key, firstname, lastname)| {
            Member{id: user_id, key: user_key, firstname, lastname}
        },
    ).unwrap();

    return Json(members);
}
