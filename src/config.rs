#![allow(non_snake_case)]

extern crate lazy_static;

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static CONFIG: OnceLock<ServerConfig> = OnceLock::new();

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub rocket_address: String,
    pub rocket_port: u16,
    pub rocket_log_level: String,

    pub db_host: String,
    pub db_port: u16,
    pub db_database: String,
    pub db_user: String,
    pub db_password: String,

    pub app_admin: String,
    pub app_session_duration_hours: u32,
    pub app_event_acceptance_auto: bool,
    pub app_event_search_date_min_year: u16,
    pub app_event_search_date_max_year: u16,
    pub app_event_search_window_min_days: u16,
    pub app_event_search_window_max_days: u16,
    pub app_event_occurrence_duration_min_minutes: u16,
    pub app_event_occurrence_duration_max_days: u16,
    pub app_event_occurrence_snap_minutes: u16,
    pub app_event_login_buffer_hours: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            rocket_address: "127.0.0.1".into(),
            rocket_port: 8000,
            rocket_log_level: "Normal".into(),

            db_host: "localhost".into(),
            db_port: 3306,
            db_database: "cptdb".into(),
            db_user: "cptdb-user".into(),
            db_password: "cptdb-password".into(),

            app_admin: "".into(),
            app_session_duration_hours: 3,
            app_event_acceptance_auto: true,
            app_event_search_date_min_year: 1000,
            app_event_search_date_max_year: 3000,
            app_event_search_window_min_days: 1,
            app_event_search_window_max_days: 800,
            app_event_occurrence_duration_min_minutes: 15,
            app_event_occurrence_duration_max_days: 14,
            app_event_occurrence_snap_minutes: 15,
            app_event_login_buffer_hours: 24,
        }
    }
}

macro_rules! env_override {
    ($cfg:expr, $field:ident, $env:literal, $ty:ty) => {
        if let Ok(v) = std::env::var($env) {
            match v.parse::<$ty>() {
                Ok(v) => $cfg.$field = v,
                Err(_) => (),
            }
        }
    };
}

impl ServerConfig {
    fn apply_env_overrides(&mut self) {
        env_override!(self, rocket_address, "CPT_ROCKET_ADDRESS", String);
        env_override!(self, rocket_port, "CPT_ROCKET_PORT", u16);
        env_override!(self, rocket_log_level, "CPT_ROCKET_LOG_LEVEL", String);
        env_override!(self, db_host, "CPT_DB_HOST", String);
        env_override!(self, db_port, "CPT_DB_PORT", u16);
        env_override!(self, db_database, "CPT_DB_DATABASE", String);
        env_override!(self, db_user, "CPT_DB_USER", String);
        env_override!(self, db_password, "CPT_DB_PASSWORD", String);
        env_override!(self, app_admin, "CPT_APP_ADMIN", String);
    }
}

pub fn read_config() {
    let path = crate::fs::get_config_path();
    let mut server_conf: ServerConfig = confy::load_path(path).unwrap();
    server_conf.apply_env_overrides();

    if crate::common::validate_user_key(&server_conf.app_admin).is_err() {
        server_conf.app_admin = "".to_string();
    }

    println!("Rocket settings");
    println!("    => address: {:?}", server_conf.rocket_address);
    println!("    => port: {:?}", server_conf.rocket_port);
    println!("    => log level: {:?}", server_conf.rocket_log_level);

    println!("Database settings");
    println!("    => host: {:?}", server_conf.db_host);
    println!("    => port: {:?}", server_conf.db_port);
    println!("    => database: {:?}", server_conf.db_database);
    println!("    => user: {:?}", server_conf.db_user);

    println!("Server settings");
    println!("    => admin: {:?}", server_conf.app_admin);
    println!(
        "    => session_duration_hour: {:?}",
        server_conf.app_session_duration_hours
    );
    println!(
        "    => event_acceptance_auto: {:?}",
        server_conf.app_event_acceptance_auto
    );
    println!(
        "    => event_search_date_min_year: {:?}",
        server_conf.app_event_search_date_min_year
    );
    println!(
        "    => event_search_date_max_year: {:?}",
        server_conf.app_event_search_date_max_year
    );
    println!(
        "    => event_search_window_min_days: {:?}",
        server_conf.app_event_search_window_min_days
    );
    println!(
        "    => event_search_window_max_days: {:?}",
        server_conf.app_event_search_window_max_days
    );
    println!(
        "    => event_occurrence_duration_min_minutes: {:?}",
        server_conf.app_event_occurrence_duration_min_minutes
    );
    println!(
        "    => event_occurrence_duration_max_days: {:?}",
        server_conf.app_event_occurrence_duration_max_days
    );
    println!(
        "    => event_occurrence_snap_minutes: {:?}",
        server_conf.app_event_occurrence_snap_minutes
    );
    println!(
        "    => event_login_buffer_hours: {:?}",
        server_conf.app_event_login_buffer_hours
    );

    let _ = CONFIG.set(server_conf);
}

/*
 * GLOBAL CONFIG GETTERS
 */

pub fn DB_NAME() -> String {
    CONFIG.get().unwrap().db_database.to_string()
}

pub fn DB_URL() -> String {
    format!(
        "mysql://{user}:{password}@{host}:{port}/{database}",
        host = CONFIG.get().unwrap().db_host,
        port = CONFIG.get().unwrap().db_port,
        database = CONFIG.get().unwrap().db_database,
        user = CONFIG.get().unwrap().db_user,
        password = CONFIG.get().unwrap().db_password,
    )
}

pub fn ROCKET_CONFIG() -> rocket::config::Config {
    rocket::Config {
        address: CONFIG.get().unwrap().rocket_address.parse().unwrap(),
        port: CONFIG.get().unwrap().rocket_port,
        log_level: CONFIG.get().unwrap().rocket_log_level.parse().unwrap(),
        ..rocket::Config::default()
    }
}

pub fn ADMIN_USER() -> Option<&'static str> {
    match &CONFIG.get().unwrap().app_admin {
        user if user.is_empty() => None,
        user => Some(user),
    }
}

pub fn SESSION_DURATION() -> chrono::Duration {
    chrono::Duration::hours(CONFIG.get().unwrap().app_session_duration_hours as i64)
}

pub fn EVENT_ACCEPTENCE_AUTO() -> bool {
    CONFIG.get().unwrap().app_event_acceptance_auto
}

pub fn EVENT_SEARCH_DATE_MIN() -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::from(
        chrono::NaiveDate::from_ymd_opt(CONFIG.get().unwrap().app_event_search_date_min_year as i32, 1, 1).unwrap(),
    )
}

pub fn EVENT_SEARCH_DATE_MAX() -> chrono::NaiveDateTime {
    chrono::NaiveDateTime::from(
        chrono::NaiveDate::from_ymd_opt(CONFIG.get().unwrap().app_event_search_date_max_year as i32, 1, 1).unwrap(),
    )
}

pub fn EVENT_SEARCH_WINDOW_MIN() -> chrono::Duration {
    chrono::Duration::days(CONFIG.get().unwrap().app_event_search_window_min_days as i64)
}

pub fn EVENT_SEARCH_WINDOW_MAX() -> chrono::Duration {
    chrono::Duration::days(CONFIG.get().unwrap().app_event_search_window_max_days as i64)
}

pub fn EVENT_OCCURRENCE_DURATION_MIN() -> chrono::Duration {
    chrono::Duration::minutes(CONFIG.get().unwrap().app_event_occurrence_duration_min_minutes as i64)
}

pub fn EVENT_OCCURRENCE_DURATION_MAX() -> chrono::Duration {
    chrono::Duration::days(CONFIG.get().unwrap().app_event_occurrence_duration_max_days as i64)
}

pub fn EVENT_OCCURRENCE_SNAP() -> chrono::Duration {
    chrono::Duration::minutes(CONFIG.get().unwrap().app_event_occurrence_snap_minutes as i64)
}

pub fn EVENT_LOGIN_BUFFER() -> chrono::Duration {
    chrono::Duration::hours(CONFIG.get().unwrap().app_event_login_buffer_hours as i64)
}
