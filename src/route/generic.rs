use crate::error::Result;

pub fn index() -> &'static str {
    "Welcome to the CPT server."
}

pub fn status() -> Result<()> {
    Ok(())
}
