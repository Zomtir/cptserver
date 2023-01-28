//#![feature(try_blocks)]

use rocket_cors::{AllowedHeaders, AllowedOrigins};

use std::collections::HashSet;

extern crate mysql_common;

mod config;
mod db;
mod db_course;
mod db_ranking;
mod db_slot;
mod db_team;
mod db_term;
mod db_user;

mod api;
mod common;
mod session;
pub mod clock;
mod route_login;

mod route_anon;

mod route_user_member;
mod route_user_admin;

mod route_course_member;
mod route_course_moderator;
mod route_course_admin;

mod route_class_admin;
mod route_class_moderator;

mod route_event_member;
mod route_event_owner;
mod route_event_admin;

mod route_ranking_member;
mod route_ranking_admin;

mod route_team_admin;
mod route_term_admin;

mod route_slot;

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
            route_login::user_login,
            route_login::slot_login,
            route_login::location_login,
            route_user_admin::user_list,
            route_user_admin::user_create,
            route_user_admin::user_edit,
            route_user_admin::user_edit_password,
            route_user_admin::user_delete,
            route_user_member::user_info,
            route_user_member::user_right,
            route_user_member::user_password,
            route_user_member::user_list,
            route_course_admin::course_list,
            route_course_admin::course_create,
            route_course_admin::course_edit,
            route_course_admin::course_delete,
            route_course_admin::course_moderator_list,
            route_course_admin::course_moderator_add,
            route_course_admin::course_moderator_remove,
            route_course_member::course_availiblity,
            route_course_member::course_class_list,
            route_course_moderator::course_responsiblity,
            route_course_moderator::course_moderator_list,
            route_course_moderator::course_moderator_add,
            route_course_moderator::course_moderator_remove,
            route_class_admin::class_list,
            route_class_admin::class_create,
            route_class_admin::class_edit,
            route_class_admin::class_edit_password,
            route_class_admin::class_delete,
            route_class_admin::class_owner_list,
            route_class_admin::class_owner_add,
            route_class_admin::class_owner_remove,
            route_class_admin::class_participant_list,
            route_class_admin::class_participant_add,
            route_class_admin::class_participant_remove,
            route_class_moderator::class_list,
            route_class_moderator::class_create,
            route_class_moderator::class_edit,
            route_class_moderator::class_edit_password,
            route_class_moderator::class_delete,
            route_event_admin::event_list,
            route_event_admin::event_accept,
            route_event_admin::event_deny,
            route_event_admin::event_cancel,
            route_event_member::event_list,
            route_event_member::event_create,
            route_event_member::event_owner_condition,
            route_event_owner::event_edit,
            route_event_owner::event_edit_password,
            route_event_owner::event_delete,
            route_event_owner::event_submit,
            route_event_owner::event_withdraw,
            route_event_owner::event_cancel,
            route_event_owner::event_recycle,
            route_event_owner::event_owner_list,
            route_event_owner::event_owner_add,
            route_event_owner::event_owner_remove,
            route_team_admin::team_list,
            route_team_admin::team_create,
            route_team_admin::team_edit,
            route_team_admin::team_delete,
            route_team_admin::team_enrol,
            route_team_admin::team_dismiss,
            route_term_admin::term_list,
            route_term_admin::term_create,
            route_term_admin::term_edit,
            route_term_admin::term_delete,
            route_ranking_admin::ranking_list,
            route_ranking_admin::ranking_create,
            route_ranking_admin::ranking_edit,
            route_ranking_admin::ranking_delete,
            route_ranking_member::ranking_list,
            route_ranking_member::ranking_summary,
            route_slot::slot_info,
            route_slot::slot_candidates,
            route_slot::slot_participant_list,
            route_slot::slot_participant_add,
            route_slot::slot_participant_remove,
        ])
        .attach(cors)
}
