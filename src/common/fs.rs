use crate::error::Error;
use regex::Regex;
use std::path::PathBuf;

pub fn local_path(partial_path: &str) -> Result<PathBuf, Error> {
    let exe_path = std::env::current_exe().map_err(|_| Error::Default)?;
    let exe_folder = exe_path.parent().ok_or(Error::Default)?;
    Ok(exe_folder.join(partial_path))
}

pub fn validate_path(partial_path: &str) -> Result<(), Error> {
    match Regex::new(r"^[a-zA-Z0-9_.+\-]+$") {
        Err(..) => Err(Error::RegexError),
        Ok(regex) => match regex.is_match(partial_path) {
            false => Err(Error::Default),
            true => Ok(()),
        },
    }
}
