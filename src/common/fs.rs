use crate::error::Error;
use std::path::PathBuf;

pub fn local_path(partial_path: &str) -> Result<PathBuf, Error> {
    let exe_path = std::env::current_exe().map_err(|_| Error::Default)?;
    let exe_folder = exe_path.parent().ok_or(Error::Default)?;
    Ok(exe_folder.join(partial_path))
}
