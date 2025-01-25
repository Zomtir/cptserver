#![allow(clippy::too_many_arguments)]

use rocket_cors::{AllowedHeaders, AllowedOrigins};

use std::collections::HashSet;

extern crate mysql_common;

mod common;
mod config;
mod db;
mod error;
mod route;
mod session;
mod utils;

#[rocket::get("/")]
fn index() -> &'static str {
    "Welcome to the CPT server."
}

#[rocket::launch]
fn rocket() -> _ {
    config::readConfig();

    if utils::db::init_db_pool().is_err() {
        panic!("Database pool initialization failed")
    };

    let mut conn = match utils::db::get_db_conn() {
        Ok(conn) => conn,
        Err(_) => panic!("Database connection failed"),
    };

    if db::migrate_scheme(&mut conn).is_err() {
        panic!("Database update failed")
    };

    // Promote an admin user, if demanded by the config, and make him session admin
    if let Some(admin) = crate::config::ADMIN_USER() {
        // Create the user, if not existing
        let elevate: bool = match crate::db::user::user_created_true(&mut conn, admin) {
            // Database query failed, do not elevate
            Err(_) => false,
            // User is missing, create him
            Ok(false) => {
                let mut user = crate::common::User::from_info(
                    0,
                    admin.clone(),
                    "Placeholder".to_string(),
                    "Placeholder".to_string(),
                    None,
                );
                // Elevate unless database query failed
                crate::db::user::user_create(&mut conn, &mut user).is_ok()
            }
            // User is existing, elevate him
            Ok(true) => true,
        };

        if elevate {
            *crate::session::ADMINSESSION.lock().unwrap() = Some(admin.clone())
        };
    }

    let rocket_config = crate::config::ROCKET_CONFIG();

    // CORS
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods = vec![
        rocket::http::Method::Head,
        rocket::http::Method::Get,
        rocket::http::Method::Post,
        rocket::http::Method::Delete,
    ]
    .into_iter()
    .map(From::from)
    .collect();
    let allowed_headers = AllowedHeaders::some(&["Token", "Accept", "Content-Type"]);
    let expose_headers = HashSet::from(["Error-URI".to_string(), "Error-MSG".to_string()]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers,
        allow_credentials: true,
        expose_headers,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::custom(&rocket_config)
        //.register(catchers![catchers::user_not_found])
        .mount(
            "/",
            rocket::routes![
                index,
                route::anon::status,
                route::anon::location_list,
                route::anon::organisation_list,
                route::anon::skill_list,
                route::anon::club_list,
                route::anon::club_image,
                route::anon::course_list,
                route::anon::user_salt,
                route::login::user_login,
                route::login::event_login,
                route::login::course_login,
                route::login::location_login,
                route::admin::user::user_list,
                route::admin::user::user_detailed,
                route::admin::user::user_create,
                route::admin::user::user_edit,
                route::admin::user::user_edit_password,
                route::admin::user::user_delete,
                route::admin::user::user_bank_account_create,
                route::admin::user::user_bank_account_edit,
                route::admin::user::user_bank_account_delete,
                route::admin::user::user_license_main_create,
                route::admin::user::user_license_extra_create,
                route::admin::user::user_license_main_edit,
                route::admin::user::user_license_extra_edit,
                route::admin::user::user_license_main_delete,
                route::admin::user::user_license_extra_delete,
                route::regular::user::user_info,
                route::regular::user::user_right,
                route::regular::user::user_password,
                route::regular::user::user_list,
                route::admin::club::club_list,
                route::admin::club::club_info,
                route::admin::club::club_create,
                route::admin::club::club_edit,
                route::admin::club::club_delete,
                route::admin::club::statistic_terms,
                route::admin::club::statistic_members,
                route::admin::club::statistic_team,
                route::admin::club::statistic_organisation,
                route::admin::club::club_statistic_user_leader,
                route::admin::club::club_statistic_user_participant,
                route::admin::club::club_statistic_user_supporter,
                route::admin::course::course_list,
                route::admin::course::course_create,
                route::admin::course::course_edit,
                route::admin::course::course_delete,
                route::admin::course::course_event_list,
                route::admin::course::course_requirement_list,
                route::admin::course::course_requirement_add,
                route::admin::course::course_requirement_remove,
                route::admin::course::course_club_info,
                route::admin::course::course_club_edit,
                route::admin::course::course_statistic_class,
                route::admin::course::course_statistic_leader,
                route::admin::course::course_statistic_leader1,
                route::admin::course::course_statistic_supporter,
                route::admin::course::course_statistic_supporter1,
                route::admin::course::course_statistic_participant,
                route::admin::course::course_statistic_participant1,
                route::admin::course::moderator::course_moderator_list,
                route::admin::course::moderator::course_moderator_add,
                route::admin::course::moderator::course_moderator_remove,
                route::admin::course::leader::sieve_list,
                route::admin::course::leader::sieve_edit,
                route::admin::course::leader::sieve_remove,
                route::admin::course::supporter::sieve_list,
                route::admin::course::supporter::sieve_edit,
                route::admin::course::supporter::sieve_remove,
                route::admin::course::participant::sieve_list,
                route::admin::course::participant::sieve_edit,
                route::admin::course::participant::sieve_remove,
                route::regular::course::course_availability,
                route::moderator::course::course_responsibility,
                route::moderator::course::course_moderator_list,
                route::moderator::course::course_moderator_add,
                route::moderator::course::course_moderator_remove,
                route::admin::event::event_list,
                route::admin::event::event_info,
                route::admin::event::event_credential,
                route::admin::event::event_create,
                route::admin::event::event_edit,
                route::admin::event::event_password_edit,
                route::admin::event::event_course_info,
                route::admin::event::event_course_edit,
                route::admin::event::event_delete,
                route::admin::event::event_accept,
                route::admin::event::event_reject,
                route::admin::event::event_suspend,
                route::admin::event::event_withdraw,
                route::admin::event::statistic_packlist,
                route::admin::event::statistic_organisation,
                route::admin::event::owner::owner_list,
                route::admin::event::owner::owner_add,
                route::admin::event::owner::owner_remove,
                route::admin::event::leader::registration_list,
                route::admin::event::leader::filter_list,
                route::admin::event::leader::filter_edit,
                route::admin::event::leader::filter_remove,
                route::admin::event::leader::presence_pool,
                route::admin::event::leader::presence_list,
                route::admin::event::leader::presence_add,
                route::admin::event::leader::presence_remove,
                route::admin::event::supporter::registration_list,
                route::admin::event::supporter::filter_list,
                route::admin::event::supporter::filter_edit,
                route::admin::event::supporter::filter_remove,
                route::admin::event::supporter::presence_pool,
                route::admin::event::supporter::presence_list,
                route::admin::event::supporter::presence_add,
                route::admin::event::supporter::presence_remove,
                route::admin::event::participant::registration_list,
                route::admin::event::participant::filter_list,
                route::admin::event::participant::filter_edit,
                route::admin::event::participant::filter_remove,
                route::admin::event::participant::presence_pool,
                route::admin::event::participant::presence_list,
                route::admin::event::participant::presence_add,
                route::admin::event::participant::presence_remove,
                route::moderator::event::event_list,
                route::moderator::event::event_create,
                route::moderator::event::event_edit,
                route::moderator::event::event_edit_password,
                route::moderator::event::event_delete,
                route::regular::event::event_list,
                route::regular::event::event_create,
                route::regular::event::event_owner_true,
                route::regular::event::event_leader_registration_info,
                route::regular::event::event_leader_registration_edit,
                route::regular::event::event_leader_presence_true,
                route::regular::event::event_leader_presence_add,
                route::regular::event::event_leader_presence_remove,
                route::regular::event::event_supporter_registration_info,
                route::regular::event::event_supporter_registration_edit,
                route::regular::event::event_supporter_presence_true,
                route::regular::event::event_supporter_presence_add,
                route::regular::event::event_supporter_presence_remove,
                route::regular::event::event_participant_registration_info,
                route::regular::event::event_participant_registration_edit,
                route::regular::event::event_participant_presence_true,
                route::regular::event::event_participant_presence_add,
                route::regular::event::event_participant_presence_remove,
                route::regular::event::event_bookmark_true,
                route::regular::event::event_bookmark_edit,
                route::owner::event::event_list,
                route::owner::event::event_info,
                route::owner::event::event_edit,
                route::owner::event::event_password_edit,
                route::owner::event::event_delete,
                route::owner::event::event_submit,
                route::owner::event::event_withdraw,
                route::owner::event::event_course_info,
                route::owner::event::event_course_edit,
                route::owner::event::owner::event_owner_list,
                route::owner::event::owner::event_owner_add,
                route::owner::event::owner::event_owner_remove,
                route::owner::event::leader::registration_list,
                route::owner::event::leader::filter_list,
                route::owner::event::leader::filter_edit,
                route::owner::event::leader::filter_remove,
                route::owner::event::leader::presence_pool,
                route::owner::event::leader::presence_list,
                route::owner::event::leader::presence_add,
                route::owner::event::leader::presence_remove,
                route::owner::event::supporter::registration_list,
                route::owner::event::supporter::filter_list,
                route::owner::event::supporter::filter_edit,
                route::owner::event::supporter::filter_remove,
                route::owner::event::supporter::presence_pool,
                route::owner::event::supporter::presence_list,
                route::owner::event::supporter::presence_add,
                route::owner::event::supporter::presence_remove,
                route::owner::event::participant::registration_list,
                route::owner::event::participant::filter_list,
                route::owner::event::participant::filter_edit,
                route::owner::event::participant::filter_remove,
                route::owner::event::participant::presence_pool,
                route::owner::event::participant::presence_list,
                route::owner::event::participant::presence_add,
                route::owner::event::participant::presence_remove,
                route::admin::location::location_list,
                route::admin::location::location_create,
                route::admin::location::location_edit,
                route::admin::location::location_delete,
                route::admin::organisation::organisation_list,
                route::admin::organisation::organisation_info,
                route::admin::organisation::organisation_create,
                route::admin::organisation::organisation_edit,
                route::admin::organisation::organisation_delete,
                route::admin::organisation::affiliation_list,
                route::admin::organisation::affiliation_info,
                route::admin::organisation::affiliation_create,
                route::admin::organisation::affiliation_edit,
                route::admin::organisation::affiliation_delete,
                route::regular::inventory::possession_list,
                route::regular::inventory::itemcat_list,
                route::admin::inventory::item_list,
                route::admin::inventory::item_create,
                route::admin::inventory::item_edit,
                route::admin::inventory::item_delete,
                route::admin::inventory::itemcat_list,
                route::admin::inventory::itemcat_create,
                route::admin::inventory::itemcat_edit,
                route::admin::inventory::itemcat_delete,
                route::admin::inventory::stock_list,
                route::admin::inventory::stock_create,
                route::admin::inventory::stock_edit,
                route::admin::inventory::stock_delete,
                route::admin::inventory::item_loan,
                route::admin::inventory::item_return,
                route::admin::inventory::item_handout,
                route::admin::inventory::item_restock,
                route::admin::inventory::possession_list,
                route::admin::inventory::possession_create,
                route::admin::inventory::possession_delete,
                route::admin::skill::skill_list,
                route::admin::skill::skill_create,
                route::admin::skill::skill_edit,
                route::admin::skill::skill_delete,
                route::admin::team::team_list,
                route::admin::team::team_info,
                route::admin::team::team_create,
                route::admin::team::team_edit,
                route::admin::team::team_right_edit,
                route::admin::team::team_delete,
                route::admin::team::team_member_list,
                route::admin::team::team_member_add,
                route::admin::team::team_member_remove,
                route::regular::team::team_list,
                route::admin::club::term::term_list,
                route::admin::club::term::term_create,
                route::admin::club::term::term_edit,
                route::admin::club::term::term_delete,
                route::admin::competence::competence_list,
                route::admin::competence::competence_create,
                route::admin::competence::competence_edit,
                route::admin::competence::competence_delete,
                route::regular::competence::competence_list,
                route::regular::competence::competence_summary,
                route::service::event::event_info,
                route::service::event::event_note_edit,
                route::service::event::event_leader_presence_pool,
                route::service::event::event_leader_presence_list,
                route::service::event::event_leader_presence_add,
                route::service::event::event_leader_presence_remove,
                route::service::event::event_supporter_presence_pool,
                route::service::event::event_supporter_presence_list,
                route::service::event::event_supporter_presence_add,
                route::service::event::event_supporter_presence_remove,
                route::service::event::event_participant_presence_pool,
                route::service::event::event_participant_presence_list,
                route::service::event::event_participant_presence_add,
                route::service::event::event_participant_presence_remove,
            ],
        )
        .attach(cors)
}
