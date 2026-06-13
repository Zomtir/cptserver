use crate::error::{Error, ErrorKind, Result};
use regex::Regex;

use std::{path::PathBuf, sync::OnceLock};

static PATH_CONFIG: OnceLock<PathBuf> = OnceLock::new();
static PATH_SQL: OnceLock<PathBuf> = OnceLock::new();
static PATH_RESOURCES: OnceLock<PathBuf> = OnceLock::new();
static PATH_DATA: OnceLock<PathBuf> = OnceLock::new();

pub fn validate_path(partial_path: &str) -> Result<()> {
    match Regex::new(r"^[a-zA-Z0-9_.+\-]+$") {
        Err(..) => Err(Error::new(ErrorKind::Regex, "Path regex failed")),
        Ok(regex) => match regex.is_match(partial_path) {
            false => Err(Error::new(ErrorKind::Default, "Path is invalid")),
            true => Ok(()),
        },
    }
}

pub fn init_paths(path_home: Option<PathBuf>) {
    let path_home = match path_home {
        Some(dir) if !dir.as_os_str().is_empty() => dir,
        _ => PathBuf::from("."),
    };

    let path_config = std::env::var("CPT_PATH_CONFIG")
        .map(PathBuf::from)
        .unwrap_or_else(|_| path_home.join("cptserver.toml"));
    let path_sql = std::env::var("CPT_PATH_SQL")
        .map(PathBuf::from)
        .unwrap_or_else(|_| path_home.join("sql"));
    let path_resources = std::env::var("CPT_PATH_RESOURCES")
        .map(PathBuf::from)
        .unwrap_or_else(|_| path_home.join("resources"));
    let path_data = std::env::var("CPT_PATH_DATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| path_home.join("data"));

    let _ = PATH_CONFIG.set(path_config);
    let _ = PATH_SQL.set(path_sql);
    let _ = PATH_RESOURCES.set(path_resources);
    let _ = PATH_DATA.set(path_data);
}

pub fn get_config_path() -> &'static PathBuf {
    PATH_CONFIG.get().unwrap()
}

pub fn get_sql_path() -> &'static PathBuf {
    PATH_SQL.get().unwrap()
}

pub fn get_resources_path() -> &'static PathBuf {
    PATH_RESOURCES.get().unwrap()
}

pub fn get_data_path() -> &'static PathBuf {
    PATH_DATA.get().unwrap()
}
