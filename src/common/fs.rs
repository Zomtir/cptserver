use crate::error::ErrorKind;
use regex::Regex;
use std::path::PathBuf;

pub fn local_path(partial_path: &str) -> PathBuf {
    let dir: String = match std::env::var("CPTSERVER_CONFIG") {
        Ok(dir) if !dir.is_empty() => dir,
        _ => ".".to_string(),
    };

    PathBuf::from(dir).join(partial_path)
}

pub fn validate_path(partial_path: &str) -> Result<(), ErrorKind> {
    match Regex::new(r"^[a-zA-Z0-9_.+\-]+$") {
        Err(..) => Err(ErrorKind::RegexError),
        Ok(regex) => match regex.is_match(partial_path) {
            false => Err(ErrorKind::Default),
            true => Ok(()),
        },
    }
}
