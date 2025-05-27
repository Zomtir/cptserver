use crate::error::ErrorKind;
use regex::Regex;
use std::path::PathBuf;

pub fn local_path(partial_path: &str) -> Result<PathBuf, ErrorKind> {
    let exe_path = std::env::current_exe().map_err(|_| ErrorKind::Default)?;
    let exe_folder = exe_path.parent().ok_or(ErrorKind::Default)?;
    Ok(exe_folder.join(partial_path))
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
