#![allow(clippy::too_many_arguments)]

use axum::Router;
use axum::routing::{get, post, put, delete, patch};
use axum::http::Method;
use axum::http::header::{self, HeaderName};
use tower_http::cors::{Any, CorsLayer};

extern crate mysql_common;

mod common;
mod config;
mod db;
mod error;
mod fs;
mod permission;
mod route;
mod session;
mod utils;

#[derive(Clone)]
struct AppState {
    db: mysql::Pool,
}

#[tokio::main]
async fn main() -> Option<()> {
    let path_home_exe: Option<std::path::PathBuf> = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));
    let path_home_env = std::env::var("CPT_PATH_HOME").ok().map(|p| std::path::PathBuf::from(p));
    let path_home = path_home_env.or(path_home_exe);

    fs::init_paths(path_home);

    config::read_config();

    if utils::db::init_db_pool().is_err() {
        panic!("Database pool initialization failed")
    };

    let mut conn = match utils::db::get_db_conn() {
        Ok(conn) => conn,
        Err(_) => panic!("Database connection failed"),
    };

    if db::migrate_scheme(&mut conn, &crate::config::DB_NAME()).is_err() {
        panic!("Database update failed")
    };

    if permission::promote_user_to_admin(&mut conn).is_err() {
        panic!("Admin elevation failed")
    };

    // Setup AppState
    let url = crate::config::DB_URL();
    let pool = mysql::Pool::new(mysql::Opts::from_url(&url)?)?;
    let app_state = AppState { db: pool };

    // CORS
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::HEAD,
        ])
        .allow_headers([
            header::ACCEPT,
            header::CONTENT_TYPE,
            // TODO replace by Authorization header
            HeaderName::from_static("token"),
        ])
        .expose_headers([
            HeaderName::from_static("error-uri"),
            HeaderName::from_static("error-msg"),
        ])
        // TODO: Enable credentials when token is retired
        .allow_credentials(false);

    // Router
    let app = Router::new()
        .route("/", get(route::generic::index))
        .route("/status", get(route::generic::status))
        .route("/admin/event_owner_list", get(route::admin::event::owner::owner_list))
        .route("/admin/event_owner_add", post(route::admin::event::owner::owner_add))
        .route("/admin/event_owner_remove", delete(route::admin::event::owner::owner_remove))

        /*
        
        
        #[rocket::get("/admin/discipline_list")]
        #[rocket::post("/admin/discipline_create", format = "application/json", data = "<discipline>")]
        #[rocket::post(
            "/admin/discipline_edit?<discipline_id>",
            format = "application/json",
            data = "<discipline>"
        )]
        #[rocket::head("/admin/discipline_delete?<discipline_id>")]
        
        #[rocket::get("/admin/competence_list?<user_id>&<skill_id>&<min>&<max>")]
        #[rocket::get("/admin/competence_info?<competence_id>")]
        #[rocket::post("/admin/competence_create", format = "application/json", data = "<competence>")]
        #[rocket::post(
            "/admin/competence_edit?<competence_id>",
            format = "application/json",
            data = "<competence>"
        )]
        #[rocket::head("/admin/competence_delete?<competence_id>")]
        

        #[rocket::get("/admin/skill_list")]
        #[rocket::post("/admin/skill_create", format = "application/json", data = "<skill>")]
        #[rocket::post("/admin/skill_edit?<skill_id>", format = "application/json", data = "<skill>")]
        #[rocket::head("/admin/skill_delete?<skill_id>")]

        #[rocket::post("/user_login", format = "application/json", data = "<credit>")]
        #[rocket::post("/event_login", format = "application/json", data = "<credit>")]
        #[rocket::get("/course_login?<course_key>")]
        #[rocket::get("/location_login?<location_key>")]

        //#[rocket::get("/anon/location_list")]
        //#[rocket::get("/anon/organisation_list")]
        //#[rocket::get("/anon/skill_list")]
        //#[rocket::get("/anon/club_list")]
        //#[rocket::get("/anon/club_image?<club_id>")]
        //#[rocket::get("/anon/club_banner?<club_id>")]
        //#[rocket::get("/anon/course_list")]
        //#[rocket::get("/anon/user_salt?<user_key>")]
        //#[rocket::get("/admin/event_owner_list?<event_id>")]
        //#[rocket::head("/admin/event_owner_add?<event_id>&<user_id>")]
        //#[rocket::head("/admin/event_owner_remove?<event_id>&<user_id>")]

        #[rocket::get("/service/event_info")]
        #[rocket::post("/service/event_note_edit", format = "text/plain", data = "<note>")]
        #[rocket::get("/service/event_attendance_presence_pool?<role>")]
        #[rocket::get("/service/event_attendance_presence_list?<role>")]
        #[rocket::head("/service/event_attendance_presence_add?<user_id>&<role>")]
        #[rocket::head("/service/event_attendance_presence_remove?<user_id>&<role>")]


        #[rocket::get("/regular/user_info")]
        #[rocket::get("/regular/user_right")]
        #[rocket::get("/regular/user_password_info")]
        #[rocket::post("/regular/user_password_edit", format = "application/json", data = "<credit>")]
        #[rocket::get("/regular/user_image?<user_id>")]
        #[rocket::get("/regular/user_list")]

        #[rocket::get("/regular/team_list")]

        #[rocket::get("/regular/itemcat_list")]
        #[rocket::get("/regular/possession_list?<owned>&<club_id>")]

        #[rocket::get("/admin/user_list?<active>")]
        #[rocket::get("/admin/user_detailed?<user_id>")]
        #[rocket::post("/admin/user_create", format = "application/json", data = "<user>")]
        #[rocket::post("/admin/user_edit?<user_id>", format = "application/json", data = "<user>")]
        #[rocket::head("/admin/user_delete?<user_id>")]
        #[rocket::get("/admin/user_password_info?<user_id>")]
        #[rocket::post(
            "/admin/user_password_create?<user_id>",
            format = "application/json",
            data = "<credit>"
        )]
        #[rocket::post(
            "/admin/user_password_edit?<user_id>",
            format = "application/json",
            data = "<credit>"
        )]
        #[rocket::head("/admin/user_password_delete?<user_id>")]

        #[rocket::post(
            "/admin/term_discipline_create?<term_id>",
            format = "application/json",
            data = "<term_discipline>"
        )]
        #[rocket::post(
            "/admin/term_discipline_edit?<term_discipline_id>",
            format = "application/json",
            data = "<term_discipline>"
        )]
        #[rocket::head("/admin/term_discipline_delete?<term_discipline_id>")]


        #[rocket::get("/owner/event_list?<begin>&<end>&<location_id>&<occurrence>&<acceptance>")]
        #[rocket::get("/owner/event_info?<event_id>")]
        #[rocket::post("/owner/event_edit?<event_id>", format = "application/json", data = "<event>")]
        #[rocket::post("/owner/event_password_edit?<event_id>", format = "text/plain", data = "<password>")]
        #[rocket::get("/owner/event_course_info?<event_id>")]
        #[rocket::head("/owner/event_course_edit?<event_id>&<course_id>")]
        #[rocket::head("/owner/event_submit?<event_id>")]
        #[rocket::head("/owner/event_withdraw?<event_id>")]
        #[rocket::head("/owner/event_delete?<event_id>")]


        #[rocket::get("/admin/location_list")]
        #[rocket::post("/admin/location_create", format = "application/json", data = "<location>")]
        #[rocket::post(
            "/admin/location_edit?<location_id>",
            format = "application/json",
            data = "<location>"
        )]
        #[rocket::head("/admin/location_delete?<location_id>")]


        #[rocket::get("/admin/team_list")]
        #[rocket::get("/admin/team_info?<team_id>")]
        #[rocket::post("/admin/team_create", format = "application/json", data = "<team>")]
        #[rocket::post("/admin/team_edit?<team_id>", format = "application/json", data = "<team>")]
        #[rocket::post("/admin/team_right_edit?<team_id>", format = "application/json", data = "<right>")]
        #[rocket::head("/admin/team_delete?<team_id>")]
        #[rocket::get("/admin/team_member_list?<team_id>")]
        #[rocket::head("/admin/team_member_add?<team_id>&<user_id>")]
        #[rocket::head("/admin/team_member_remove?<team_id>&<user_id>")]


        #[rocket::get("/admin/organisation_list")]
        #[rocket::get("/admin/organisation_info?<organisation_id>")]
        #[rocket::post("/admin/organisation_create", format = "application/json", data = "<organisation>")]
        #[rocket::post(
            "/admin/organisation_edit?<organisation_id>",
            format = "application/json",
            data = "<organisation>"
        )]
        #[rocket::head("/admin/organisation_delete?<organisation_id>")]


        #[rocket::get("/admin/stock_list?<club_id>&<item_id>")]
        #[rocket::post("/admin/stock_create", format = "application/json", data = "<stock>")]
        #[rocket::post("/admin/stock_edit?<stock_id>", format = "application/json", data = "<stock>")]
        #[rocket::head("/admin/stock_delete?<stock_id>")]
        #[rocket::head("/admin/item_loan?<stock_id>&<user_id>")]
        #[rocket::head("/admin/item_return?<possession_id>")]
        #[rocket::head("/admin/item_handout?<possession_id>")]
        #[rocket::head("/admin/item_restock?<possession_id>&<stock_id>")]
        #[rocket::get("/admin/possession_list?<user_id>&<item_id>&<owned>&<club_id>")]
        #[rocket::head("/admin/possession_create?<user_id>&<item_id>")]
        #[rocket::head("/admin/possession_delete?<possession_id>")]


        #[rocket::get("/admin/club_list")]
        #[rocket::get("/admin/club_info?<club_id>")]
        #[rocket::post("/admin/club_create", format = "application/json", data = "<club>")]
        #[rocket::post("/admin/club_edit?<club_id>", format = "application/json", data = "<club>")]
        #[rocket::head("/admin/club_delete?<club_id>")]
        #[rocket::get("/admin/club_statistic_terms?<club_id>&<point_in_time>")]
        #[rocket::get("/admin/club_statistic_members?<club_id>&<point_in_time>")]
        #[rocket::get("/admin/club_statistic_team?<club_id>&<point_in_time>&<team_id>")]
        #[rocket::get("/admin/club_statistic_organisation?<club_id>&<organisation_id>&<point_in_time>")]
        #[rocket::get("/admin/club_statistic_attendance?<club_id>&<user_id>&<role>&<time_window_begin>&<time_window_end>")]

        #[rocket::get("/admin/term_list?<club_id>&<user_id>")]
        #[rocket::get("/admin/term_info?<term_id>")]
        #[rocket::post("/admin/term_create", format = "application/json", data = "<term>")]
        #[rocket::post("/admin/term_edit?<term_id>", format = "application/json", data = "<term>")]
        #[rocket::head("/admin/term_delete?<term_id>")]

        #[rocket::get("/admin/course_attendance_sieve_list?<course_id>&<role>")]
        #[rocket::head("/admin/course_attendance_sieve_edit?<course_id>&<team_id>&<role>&<access>")]
        #[rocket::head("/admin/course_attendance_sieve_remove?<course_id>&<team_id>&<role>")]

        #[rocket::get("/admin/course_list?<mod_id>&<active>&<public>")]
        #[rocket::post("/admin/course_create", format = "application/json", data = "<course>")]
        #[rocket::post("/admin/course_edit?<course_id>", format = "application/json", data = "<course>")]
        #[rocket::head("/admin/course_delete?<course_id>")]
        #[rocket::get("/admin/course_event_list?<course_id>")]
        #[rocket::get("/admin/course_requirement_list?<course_id>")]
        #[rocket::head("/admin/course_requirement_add?<course_id>&<skill_id>&<rank>")]
        #[rocket::head("/admin/course_requirement_remove?<requirement_id>")]
        #[rocket::get("/admin/course_club_info?<course_id>")]
        #[rocket::head("/admin/course_club_edit?<course_id>&<club_id>")]
        #[rocket::get("/admin/course_statistic_class?<course_id>")]
        #[rocket::get("/admin/course_statistic_attendance?<course_id>&<role>")]
        #[rocket::get("/admin/course_statistic_attendance1?<course_id>&<user_id>&<role>")]

        #[rocket::get("/admin/course_moderator_list?<course_id>")]
        #[rocket::head("/admin/course_moderator_add?<course_id>&<user_id>")]
        #[rocket::head("/admin/course_moderator_remove?<course_id>&<user_id>")]

        #[rocket::get("/admin/event_attendance_registration_list?<event_id>&<role>")]
        #[rocket::get("/admin/event_attendance_filter_list?<event_id>&<role>")]
        #[rocket::head("/admin/event_attendance_filter_edit?<event_id>&<user_id>&<role>&<access>")]
        #[rocket::head("/admin/event_attendance_filter_remove?<event_id>&<user_id>&<role>")]
        #[rocket::get("/admin/event_attendance_presence_pool?<event_id>&<role>")]
        #[rocket::get("/admin/event_attendance_presence_list?<event_id>&<role>")]
        #[rocket::head("/admin/event_attendance_presence_add?<event_id>&<user_id>&<role>")]
        #[rocket::head("/admin/event_attendance_presence_remove?<event_id>&<user_id>&<role>")]


        #[rocket::get(
            "/admin/event_list?<begin>&<end>&<location_id>&<occurrence>&<acceptance>&<course_true>&<course_id>&<owner_id>"
        )]
        #[rocket::get("/admin/event_info?<event_id>")]
        #[rocket::get("/admin/event_credential?<event_id>")]
        #[rocket::post("/admin/event_create?<course_id>", format = "application/json", data = "<event>")]
        #[rocket::post("/admin/event_edit?<event_id>", format = "application/json", data = "<event>")]
        #[rocket::post("/admin/event_password_edit?<event_id>", format = "text/plain", data = "<password>")]
        #[rocket::get("/admin/event_course_info?<event_id>")]
        #[rocket::head("/admin/event_course_edit?<event_id>&<course_id>")]
        #[rocket::head("/admin/event_delete?<event_id>")]
        #[rocket::head("/admin/event_accept?<event_id>")]
        #[rocket::head("/admin/event_reject?<event_id>")]
        #[rocket::head("/admin/event_suspend?<event_id>")]
        #[rocket::head("/admin/event_withdraw?<event_id>")]
        #[rocket::get("/admin/event_statistic_packlist?<event_id>&<skill_id>")]
        #[rocket::get("/admin/event_statistic_organisation?<event_id>&<organisation_id>")]

        #[rocket::get("/admin/user_equipment_list?<user_id>&<skill_id>&<item_id>")]
        #[rocket::get("/admin/user_equipment_info?<equipment_id>")]
        #[rocket::head("/admin/user_equipment_create?<user_id>&<skill_id>&<item_id>&<count>")]
        #[rocket::head("/admin/user_equipment_edit?<equipment_id>&<count>")]
        #[rocket::head("/admin/user_equipment_delete?<equipment_id>")]

        #[rocket::get("/admin/item_list?<category_id>")]
        #[rocket::get("/admin/item_info?<item_id>")]
        #[rocket::post("/admin/item_create", format = "application/json", data = "<item>")]
        #[rocket::post("/admin/item_edit?<item_id>", format = "application/json", data = "<item>")]
        #[rocket::head("/admin/item_delete?<item_id>")]
        #[rocket::get("/admin/itemcat_list")]
        #[rocket::post("/admin/itemcat_create", format = "application/json", data = "<itemcat>")]
        #[rocket::post("/admin/itemcat_edit?<category_id>", format = "application/json", data = "<itemcat>")]
        #[rocket::head("/admin/itemcat_delete?<category_id>")]


        #[rocket::get("/admin/affiliation_list?<user_id>&<organisation_id>")]
        #[rocket::get("/admin/affiliation_info?<user_id>&<organisation_id>")]
        #[rocket::head("/admin/affiliation_create?<user_id>&<organisation_id>")]
        #[rocket::post(
            "/admin/affiliation_edit?<user_id>&<organisation_id>",
            format = "application/json",
            data = "<affiliation>"
        )]
        #[rocket::head("/admin/affiliation_delete?<user_id>&<organisation_id>")]


        #[rocket::post(
            "/admin/user_bank_account_create?<user_id>",
            format = "application/json",
            data = "<bank_account>"
        )]
        #[rocket::post(
            "/admin/user_bank_account_edit?<user_id>",
            format = "application/json",
            data = "<bank_account>"
        )]
        #[rocket::head("/admin/user_bank_account_delete?<user_id>")]


        #[rocket::post(
            "/admin/user_license_main_create?<user_id>",
            format = "application/json",
            data = "<license>"
        )]
        #[rocket::post(
            "/admin/user_license_extra_create?<user_id>",
            format = "application/json",
            data = "<license>"
        )]
        #[rocket::post(
            "/admin/user_license_main_edit?<user_id>",
            format = "application/json",
            data = "<license>"
        )]
        #[rocket::post(
            "/admin/user_license_extra_edit?<user_id>",
            format = "application/json",
            data = "<license>"
        )]
        #[rocket::head("/admin/user_license_main_delete?<user_id>")]
        #[rocket::head("/admin/user_license_extra_delete?<user_id>")]



        #[rocket::get("/mod/course_responsibility?<active>&<public>")]
        #[rocket::get("/mod/course_moderator_list?<course_id>")]
        #[rocket::head("/mod/course_moderator_add?<course_id>&<user_id>")]
        #[rocket::head("/mod/course_moderator_remove?<course_id>&<user_id>")]


        #[rocket::get("/mod/event_list?<course_id>")]
        #[rocket::post("/mod/event_create?<course_id>", format = "application/json", data = "<event>")]
        #[rocket::post("/mod/event_edit?<event_id>", format = "application/json", data = "<event>")]
        #[rocket::post("/mod/event_edit_password?<event_id>", format = "text/plain", data = "<password>")]
        #[rocket::head("/mod/event_delete?<event_id>")]


        #[rocket::get("/owner/event_attendance_registration_list?<event_id>&<role>")]
        #[rocket::get("/owner/event_attendance_filter_list?<event_id>&<role>")]
        #[rocket::head("/owner/event_attendance_filter_edit?<event_id>&<user_id>&<role>&<access>")]
        #[rocket::head("/owner/event_attendance_filter_remove?<event_id>&<user_id>&<role>")]
        #[rocket::get("/owner/event_attendance_presence_pool?<event_id>&<role>")]
        #[rocket::get("/owner/event_attendance_presence_list?<event_id>&<role>")]
        #[rocket::head("/owner/event_attendance_presence_add?<event_id>&<user_id>&<role>")]
        #[rocket::head("/owner/event_attendance_presence_remove?<event_id>&<user_id>&<role>")]


        #[rocket::get("/regular/course_availability")]

        #[rocket::get("/owner/event_owner_list?<event_id>")]
        #[rocket::head("/owner/event_owner_add?<event_id>&<user_id>")]
        #[rocket::head("/owner/event_owner_remove?<event_id>&<user_id>")]


        #[rocket::get("/regular/event_list?<begin>&<end>&<location_id>&<occurrence>&<acceptance>&<course_true>&<course_id>")]
        #[rocket::post("/regular/event_create", format = "application/json", data = "<event>")]
        #[rocket::get("/regular/event_owner_true?<event_id>")]
        #[rocket::get("/regular/event_moderator_true?<event_id>")]
        #[rocket::get("/regular/event_attendance_presence_true?<event_id>&<role>")]
        #[rocket::head("/regular/event_attendance_presence_add?<event_id>&<role>")]
        #[rocket::head("/regular/event_attendance_presence_remove?<event_id>&<role>")]
        #[rocket::get("/regular/event_bookmark_true?<event_id>")]
        #[rocket::head("/regular/event_bookmark_edit?<event_id>&<bookmark>")]
        #[rocket::get("/regular/event_attendance_registration_info?<event_id>&<role>")]
        #[rocket::head("/regular/event_attendance_registration_edit?<event_id>&<role>&<status>")]


        #[rocket::get("/regular/competence_list")]
        #[rocket::get("/regular/competence_summary")]
    */
        .with_state(app_state)
        .layer(cors_layer);

    // Start server
    let addr = crate::config::SERVER_URL();
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

/*
            rocket::routes![
                index,
                route::anon::status,
                route::anon::location_list,
                route::anon::organisation_list,
                route::anon::skill_list,
                route::anon::club_list,
                route::anon::club_image,
                route::anon::club_banner,
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
                route::admin::user::user_delete,
                route::admin::user::user_password_info,
                route::admin::user::user_password_create,
                route::admin::user::user_password_edit,
                route::admin::user::user_password_delete,
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
                route::regular::user::user_password_info,
                route::regular::user::user_password_set,
                route::regular::user::user_list,
                route::regular::user::user_image,
                route::admin::club::club_list,
                route::admin::club::club_info,
                route::admin::club::club_create,
                route::admin::club::club_edit,
                route::admin::club::club_delete,
                route::admin::club::statistic_terms,
                route::admin::club::statistic_members,
                route::admin::club::statistic_team,
                route::admin::club::statistic_organisation,
                route::admin::club::statistic_attendance,
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
                route::admin::course::course_statistic_attendance,
                route::admin::course::course_statistic_attendance1,
                route::admin::course::moderator::course_moderator_list,
                route::admin::course::moderator::course_moderator_add,
                route::admin::course::moderator::course_moderator_remove,
                route::admin::course::attendance::sieve_list,
                route::admin::course::attendance::sieve_edit,
                route::admin::course::attendance::sieve_remove,
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
                route::admin::event::attendance::registration_list,
                route::admin::event::attendance::filter_list,
                route::admin::event::attendance::filter_edit,
                route::admin::event::attendance::filter_remove,
                route::admin::event::attendance::presence_pool,
                route::admin::event::attendance::presence_list,
                route::admin::event::attendance::presence_add,
                route::admin::event::attendance::presence_remove,
                route::moderator::event::event_list,
                route::moderator::event::event_create,
                route::moderator::event::event_edit,
                route::moderator::event::event_edit_password,
                route::moderator::event::event_delete,
                route::regular::event::event_list,
                route::regular::event::event_create,
                route::regular::event::event_owner_true,
                route::regular::event::event_moderator_true,
                route::regular::event::event_attendance_registration_info,
                route::regular::event::event_attendance_registration_edit,
                route::regular::event::event_attendance_presence_true,
                route::regular::event::event_attendance_presence_add,
                route::regular::event::event_attendance_presence_remove,
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
                route::owner::event::attendance::registration_list,
                route::owner::event::attendance::filter_list,
                route::owner::event::attendance::filter_edit,
                route::owner::event::attendance::filter_remove,
                route::owner::event::attendance::presence_pool,
                route::owner::event::attendance::presence_list,
                route::owner::event::attendance::presence_add,
                route::owner::event::attendance::presence_remove,
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
                route::admin::inventory::item_info,
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
                route::admin::inventory::user_equipment_list,
                route::admin::inventory::user_equipment_info,
                route::admin::inventory::user_equipment_create,
                route::admin::inventory::user_equipment_edit,
                route::admin::inventory::user_equipment_delete,
                route::admin::skill::skill_list,
                route::admin::skill::skill_create,
                route::admin::skill::skill_edit,
                route::admin::skill::skill_delete,
                route::admin::discipline::discipline_list,
                route::admin::discipline::discipline_create,
                route::admin::discipline::discipline_edit,
                route::admin::discipline::discipline_delete,
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
                route::admin::club::term::term_info,
                route::admin::club::term::term_create,
                route::admin::club::term::term_edit,
                route::admin::club::term::term_delete,
                route::admin::club::term_discipline::term_discipline_create,
                route::admin::club::term_discipline::term_discipline_edit,
                route::admin::club::term_discipline::term_discipline_delete,
                route::admin::competence::competence_list,
                route::admin::competence::competence_info,
                route::admin::competence::competence_create,
                route::admin::competence::competence_edit,
                route::admin::competence::competence_delete,
                route::regular::competence::competence_list,
                route::regular::competence::competence_summary,
                route::service::event::event_info,
                route::service::event::event_note_edit,
                route::service::event::event_attendance_presence_pool,
                route::service::event::event_attendance_presence_list,
                route::service::event::event_attendance_presence_add,
                route::service::event::event_attendance_presence_remove,
            ],
             */
}
