#![allow(non_snake_case)]

extern crate lazy_static;

use serde::{Deserialize, Serialize};

static mut CONFIG: Option<ServerConfig> = None;

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub rocket_address: String,
    pub rocket_port: u16,
    pub rocket_log_level: String,

    pub db_server: String,
    pub db_port: u16,
    pub db_database: String,
    pub db_user: String,
    pub db_password: String,

    pub cpt_admin: Option<String>,
    pub cpt_session_duration_hours: u32,
    pub cpt_event_acceptance_auto: bool,
    pub cpt_event_search_date_min_year: u16,
    pub cpt_event_search_date_max_year: u16,
    pub cpt_event_search_window_min_days: u16,
    pub cpt_event_search_window_max_days: u16,
    pub cpt_event_occurrence_duration_min_minutes: u16,
    pub cpt_event_occurrence_duration_max_days: u16,
    pub cpt_event_occurrence_snap_minutes: u16,
    pub cpt_event_login_buffer_hours: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            rocket_address: "127.0.0.1".into(),
            rocket_port: 8000,
            rocket_log_level: "Normal".into(),

            db_server: "localhost".into(),
            db_port: 3306,
            db_database: "cptdb".into(),
            db_user: "cptdb-user".into(),
            db_password: "cptdb-password".into(),

            cpt_admin: None,
            cpt_session_duration_hours: 3,
            cpt_event_acceptance_auto: true,
            cpt_event_search_date_min_year: 1000,
            cpt_event_search_date_max_year: 3000,
            cpt_event_search_window_min_days: 1,
            cpt_event_search_window_max_days: 800,
            cpt_event_occurrence_duration_min_minutes: 15,
            cpt_event_occurrence_duration_max_days: 14,
            cpt_event_occurrence_snap_minutes: 15,
            cpt_event_login_buffer_hours: 24,
        }
    }
}

pub fn readConfig() {
    let mut confdir: String = match std::env::var("CPTSERVER_CONFIG") {
        Err(..) => ".".to_string(),
        Ok(dir) => dir,
    };

    if confdir.is_empty() {
        confdir = ".".to_string()
    }

    let confpath = format!("{}/{}", confdir, "cptserver.toml");

    let mut server_conf: ServerConfig = confy::load_path(confpath).unwrap();

    if let Some(ref admin) = server_conf.cpt_admin {
        if crate::common::validate_user_key(&admin).is_err() {
            server_conf.cpt_admin = None;
        }
    }

    println!("Rocket settings");
    println!("    => address: {:?}", server_conf.rocket_address);
    println!("    => port: {:?}", server_conf.rocket_port);
    println!("    => log level: {:?}", server_conf.rocket_log_level);

    println!("Database settings");
    println!("    => server: {:?}", server_conf.db_server);
    println!("    => port: {:?}", server_conf.db_port);
    println!("    => database: {:?}", server_conf.db_database);
    println!("    => user: {:?}", server_conf.db_user);

    println!("Server settings");
    println!("    => admin: {:?}", server_conf.cpt_admin);
    println!("    => session_duration_hour: {:?}", server_conf.cpt_session_duration_hours);
    println!("    => event_acceptance_auto: {:?}", server_conf.cpt_event_acceptance_auto);
    println!("    => event_search_date_min_year: {:?}", server_conf.cpt_event_search_date_min_year);
    println!("    => event_search_date_max_year: {:?}", server_conf.cpt_event_search_date_max_year);
    println!("    => event_search_window_min_days: {:?}", server_conf.cpt_event_search_window_min_days);
    println!("    => event_search_window_max_days: {:?}", server_conf.cpt_event_search_window_max_days);
    println!("    => event_occurrence_duration_min_minutes: {:?}", server_conf.cpt_event_occurrence_duration_min_minutes);
    println!("    => event_occurrence_duration_max_days: {:?}", server_conf.cpt_event_occurrence_duration_max_days);
    println!("    => event_occurrence_snap_minutes: {:?}", server_conf.cpt_event_occurrence_snap_minutes);
    println!("    => event_login_buffer_hours: {:?}", server_conf.cpt_event_login_buffer_hours);

    unsafe {
        CONFIG = Some(server_conf);
    }
}

/*
 * GLOBAL CONFIG GETTERS
 */

pub fn DB_URL() -> String {
    unsafe {
        format!(
            "mysql://{user}:{password}@{server}:{port}/{database}",
            server = CONFIG.as_ref().unwrap().db_server,
            port = CONFIG.as_ref().unwrap().db_port,
            database = CONFIG.as_ref().unwrap().db_database,
            user = CONFIG.as_ref().unwrap().db_user,
            password = CONFIG.as_ref().unwrap().db_password,
        )
    }
}

pub fn ROCKET_CONFIG() -> rocket::config::Config {
    unsafe {
        rocket::Config {
            address: CONFIG.as_ref().unwrap().rocket_address.parse().unwrap(),
            port: CONFIG.as_ref().unwrap().rocket_port,
            log_level: CONFIG.as_ref().unwrap().rocket_log_level.parse().unwrap(),
            ..rocket::Config::default()
        }
    }
}

pub fn ADMIN_USER() -> Option<&'static String> {
    unsafe { CONFIG.as_ref().unwrap().cpt_admin.as_ref() }
}

pub fn SESSION_DURATION() -> chrono::Duration {
    unsafe { chrono::Duration::hours(CONFIG.as_ref().unwrap().cpt_session_duration_hours as i64) }
}

pub fn EVENT_ACCEPTENCE_AUTO() -> bool {
    unsafe { CONFIG.as_ref().unwrap().cpt_event_acceptance_auto }
}

pub fn EVENT_SEARCH_DATE_MIN() -> chrono::NaiveDateTime {
    unsafe {
        chrono::NaiveDateTime::from(
            chrono::NaiveDate::from_ymd_opt(CONFIG.as_ref().unwrap().cpt_event_search_date_min_year as i32, 1, 1)
                .unwrap(),
        )
    }
}

pub fn EVENT_SEARCH_DATE_MAX() -> chrono::NaiveDateTime {
    unsafe {
        chrono::NaiveDateTime::from(
            chrono::NaiveDate::from_ymd_opt(CONFIG.as_ref().unwrap().cpt_event_search_date_max_year as i32, 1, 1)
                .unwrap(),
        )
    }
}

pub fn EVENT_SEARCH_WINDOW_MIN() -> chrono::Duration {
    unsafe { chrono::Duration::days(CONFIG.as_ref().unwrap().cpt_event_search_window_min_days as i64) }
}

pub fn EVENT_SEARCH_WINDOW_MAX() -> chrono::Duration {
    unsafe { chrono::Duration::days(CONFIG.as_ref().unwrap().cpt_event_search_window_max_days as i64) }
}

pub fn EVENT_OCCURRENCE_DURATION_MIN() -> chrono::Duration {
    unsafe { chrono::Duration::minutes(CONFIG.as_ref().unwrap().cpt_event_occurrence_duration_min_minutes as i64) }
}

pub fn EVENT_OCCURRENCE_DURATION_MAX() -> chrono::Duration {
    unsafe { chrono::Duration::days(CONFIG.as_ref().unwrap().cpt_event_occurrence_duration_max_days as i64) }
}

pub fn EVENT_OCCURRENCE_SNAP() -> chrono::Duration {
    unsafe { chrono::Duration::minutes(CONFIG.as_ref().unwrap().cpt_event_occurrence_snap_minutes as i64) }
}

pub fn EVENT_LOGIN_BUFFER() -> chrono::Duration {
    unsafe { chrono::Duration::hours(CONFIG.as_ref().unwrap().cpt_event_login_buffer_hours as i64) }
}
