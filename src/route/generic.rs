use crate::error::Result;

pub async fn index() -> &'static str {
    "Welcome to the CPT server."
}

pub async fn status() -> Result<()> {
    Ok(())
}
