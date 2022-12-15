//#![feature(try_blocks)]

use rocket_cors::{AllowedHeaders, AllowedOrigins};

use std::collections::HashSet;

extern crate mysql_common;

mod config;
mod db;
mod db_user;
mod db_slot;
mod api;
mod common;
mod session;
pub mod clock;
mod route_login;
mod route_admin_users;
mod route_admin_courses;
mod route_admin_rankings;
mod route_admin_reservations;
mod route_admin_teams;
mod route_user;
mod route_user_course;
mod route_user_event;
mod route_slot;
mod route_anon;

#[rocket::get("/")]
fn index() -> &'static str {
    "Welcome to the CPT server."
}

#[rocket::launch]
fn rocket() -> _ {
    db::connect_db();

    // CORS
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods = vec![rocket::http::Method::Head,rocket::http::Method::Get,rocket::http::Method::Post,rocket::http::Method::Delete].into_iter().map(From::from).collect();
    let allowed_headers = AllowedHeaders::some(&["Token", "Accept", "Content-Type"]);
    let expose_headers = HashSet::from(["Error-URI".to_string(), "Error-Message".to_string()]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers,
        allow_credentials: true,
        expose_headers: expose_headers,
        ..Default::default()
    }
    .to_cors().unwrap();

    //cors.expose_headers(&["Error-URI", "Error-Message"]);

    rocket::build()
        //.register(catchers![catchers::user_not_found])
        .mount("/", rocket::routes![index,
            route_anon::status, route_anon::location_list, route_anon::branch_list, route_anon::access_list,
            route_login::user_login, route_login::slot_login, route_login::slot_autologin,
            route_admin_users::user_list, route_admin_users::user_create, route_admin_users::user_edit, route_admin_users::user_delete,
            route_admin_courses::course_list, route_admin_courses::course_create, route_admin_courses::course_edit, route_admin_courses::course_delete,
            route_admin_courses::course_moderator_list, route_admin_courses::course_mod, route_admin_courses::course_unmod,
            route_admin_rankings::ranking_list, route_admin_rankings::ranking_create, route_admin_rankings::ranking_edit, route_admin_rankings::ranking_delete,
            route_admin_reservations::reservation_list, route_admin_reservations::reservation_accept, route_admin_reservations::reservation_deny, route_admin_reservations::reservation_cancel,
            route_admin_teams::team_list, route_admin_teams::team_create, route_admin_teams::team_edit, route_admin_teams::team_delete,
            route_admin_teams::team_enrol, route_admin_teams::team_dismiss,
            route_user::user_info, route_user::user_password, route_user::user_info_rankings, route_user::user_member_list,
            route_user_course::user_course_list, route_user_course::course_slot_list,
            route_user_course::course_slot_create, route_user_course::course_slot_edit, route_user_course::course_slot_delete,
            route_user_event::event_list,
            route_user_event::event_create, route_user_event::event_edit, route_user_event::event_delete,
            route_user_event::event_submit, route_user_event::event_withdraw, route_user_event::event_cancel, route_user_event::event_recycle,
            route_user_event::event_owner_list, route_user_event::event_owner_add, route_user_event::event_owner_remove,
            route_slot::slot_info,
            route_slot::slot_candidates, route_slot::slot_participants, route_slot::slot_enrol, route_slot::slot_dismiss,
        ])
        .attach(cors)
}
