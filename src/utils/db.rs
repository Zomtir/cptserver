use crate::error::{ErrorKind, Result};
use mysql::{Pool, PooledConn};
use std::sync::OnceLock;

static DBPOOL: OnceLock<Pool> = OnceLock::new();

pub fn init_db_pool() -> Result<()> {
    let url = crate::config::DB_URL();
    let pool = mysql::Pool::new(mysql::Opts::from_url(&url)?)?;
    let _ = DBPOOL.set(pool);

    Ok(())
}

pub fn get_db_conn() -> Result<PooledConn> {
    let pool = DBPOOL
        .get()
        .or_else(|| {
            init_db_pool().ok()?;
            DBPOOL.get()
        })
        .ok_or(ErrorKind::DatabasePool)?;

    pool.get_conn().map_err(|_| ErrorKind::DatabaseConnection)
}
