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

#[rocket::get("/member/user_info")]
pub fn user_info(session: UserSession) -> Json<User> {
    Json(User::from_info(
        session.user.id,
        session.user.key,
        session.user.firstname,
        session.user.lastname,
    ))
}

#[rocket::get("/member/user_right")]
pub fn user_right(session: UserSession) -> Json<Right> {
    Json(Right{
        admin_courses: session.right.admin_courses,
        admin_inventory: session.right.admin_inventory,
        admin_rankings: session.right.admin_rankings,
        admin_event: session.right.admin_event,
        admin_teams: session.right.admin_teams,
        admin_term: session.right.admin_term,
        admin_users: session.right.admin_users,
    })
}

#[rocket::post("/member/user_password", format = "text/plain", data = "<password>")]
pub fn user_password(session: UserSession, password: String) -> Result<(), ApiError> {
    match crate::db_user::edit_user_password(&session.user.id, password) {
        None => Err(ApiError::DB_CONFLICT),
        Some(..) => Ok(()),
    }
}

#[rocket::get("/member/user_info_rankings")]
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
pub fn user_list(_session: UserSession) -> Result<Json<Vec<User>>,ApiError> {
    match crate::db_user::list_user(Some(true)) {
        None => Err(ApiError::DB_CONFLICT),
        Some(users) => Ok(Json(users)),
    }
}
