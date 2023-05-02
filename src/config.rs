#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub rocket_address: Option<String>,
    pub rocket_port: Option<u16>,
    pub rocket_log_level: Option<String>,

    pub db_server: Option<String>,
    pub db_port: Option<u16>,
    pub db_database: Option<String>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
}

impl ::std::default::Default for ServerConfig {
    fn default() -> Self {
        Self {
            rocket_address: Some("127.0.0.1".into()),
            rocket_port: Some(8000),
            rocket_log_level: Some("Normal".into()),

            db_server: Some("localhost".into()),
            db_port: Some(3306),
            db_database: Some("cptdb".into()),
            db_user: Some("cptdb-user".into()),
            db_password: Some("cptdb-password".into()),
        }
    }
}

pub fn readConfig() -> ServerConfig {
    let mut confdir: String = match std::env::var("CPTSERVER_CONFIG") {
        Err(..) => ".".to_string(),
        Ok(dir) => dir,
    };

    if confdir.is_empty() {
        confdir = ".".to_string()
    }

    let confpath = format!("{}/{}", confdir, "CptServer.toml");

    let server_conf: ServerConfig = confy::load_path(confpath).unwrap();

    println!("Rocket settings");
    println!("    => address: {:?}", server_conf.rocket_address);
    println!("    => port: {:?}", server_conf.rocket_port);
    println!("    => log level: {:?}", server_conf.rocket_log_level);

    println!("Database settings");
    println!("    => server: {:?}", server_conf.db_server);
    println!("    => port: {:?}", server_conf.db_port);
    println!("    => database: {:?}", server_conf.db_database);
    println!("    => user: {:?}", server_conf.db_user);

    return server_conf;
}

/*
 * GLOBAL CONFIG FLAGS
 */

// Rust/chrono does not support constant contructors for Duration atm, that's why there are functions rather than static constants

pub static CONFIG_RESERVATION_AUTO_CHECK: bool = false;
//pub static CONFIG_COURSE_MODERATOR_PROMOTION : bool = true; // TODO

pub fn CONFIG_SLOT_DATE_MIN() -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::from_ymd_opt(1000, 1, 1)
}

pub fn CONFIG_SLOT_DATE_MAX() -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::from_ymd_opt(3000, 1, 1)
}

pub fn CONFIG_SLOT_LIST_TIME_MIN() -> chrono::Duration {
    chrono::Duration::days(1)
}

pub fn CONFIG_SLOT_LIST_TIME_MAX() -> chrono::Duration {
    chrono::Duration::days(366)
}

pub fn CONFIG_SLOT_AUTOLOGIN_TIME() -> chrono::Duration {
    chrono::Duration::hours(24)
}

pub fn CONFIG_SLOT_WINDOW_MINIMUM() -> chrono::Duration {
    chrono::Duration::minutes(15)
}

pub fn CONFIG_SLOT_WINDOW_SNAP() -> chrono::Duration {
    chrono::Duration::minutes(15)
}
