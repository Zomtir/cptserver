use rocket::http::Status;
use rocket::serde::json::Json;

use mysql::{PooledConn, params};
use mysql::prelude::{Queryable};

use crate::api::ApiError;
use crate::db::get_pool_conn;
use crate::session::UserSession;
use crate::common::{User, Ranking, Branch, Right};

/*
 * ROUTES
 */

#[rocket::get("/user_info")]
pub fn user_info(session: UserSession) -> Json<User> {
    Json(User::from_info(
        session.user.id,
        session.user.key,
        session.user.firstname,
        session.user.lastname,
    ))
}

#[rocket::get("/user_right")]
pub fn user_right(session: UserSession) -> Json<Right> {
    Json(Right{
        admin_courses: session.right.admin_courses,
        admin_inventory: session.right.admin_inventory,
        admin_rankings: session.right.admin_rankings,
        admin_reservations: session.right.admin_reservations,
        admin_teams: session.right.admin_teams,
        admin_users: session.right.admin_users,
    })
}

#[rocket::post("/user_password", format = "text/plain", data = "<password>")]
pub fn user_password(session: UserSession, password: String) -> Result<Status, ApiError> {
    let bpassword : Vec<u8> = match crate::common::verify_password(&password){
        Some(bpassword) => bpassword,
        None => return Err(ApiError::USER_BAD_PASSWORD),
    };
    
    let pepper : Vec<u8> = crate::common::random_bytes(16);
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
        user: session.user.clone(),
        branch: Branch{id: branch_id, key: branch_key, title: branch_title},
        rank, date,
        judge: User::from_info(judge_id, judge_key, judge_firstname, judge_lastname),
      };

    let rankings = conn.exec_map(
        &stmt,
        params! { "user_id" => session.user.id },
        &map,
    ).unwrap();

    return Json(rankings);
}

// TODO only active members
#[rocket::get("/member/user_list")]
pub fn user_list(_session: UserSession) -> Json<Vec<User>> {
    let mut conn : PooledConn = get_pool_conn();
    let stmt = conn.prep("SELECT user_id, user_key, firstname, lastname FROM users").unwrap();

    let users = conn.exec_map(
        &stmt,
        params::Params::Empty,
        |(user_id, user_key, firstname, lastname)| {
            User::from_info(user_id, user_key, firstname, lastname)
        },
    ).unwrap();

    return Json(users);
}
