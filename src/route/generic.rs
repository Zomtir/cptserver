use crate::error::Result;

pub async fn index() -> &'static str {
    "Welcome to the CPT server."
}

pub async fn status() -> Result<()> {
    Ok(())
}

pub async fn version() -> Result<String> {
    Ok(env!("CARGO_PKG_VERSION").into())
}
