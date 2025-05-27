use cptserver::error::ErrorKind;
use mysql::{Pool, PooledConn};
use std::sync::OnceLock;

mod config;

static DBTPOOL: OnceLock<Pool> = OnceLock::new();

pub fn init_dbt_pool() -> Result<(), ErrorKind> {
    let url = dbt_url();
    let pool = mysql::Pool::new(mysql::Opts::from_url(&url)?)?;
    let _ = DBTPOOL.set(pool);

    Ok(())
}

pub fn get_dbt_conn() -> Result<PooledConn, ErrorKind> {
    let pool = DBTPOOL
        .get()
        .or_else(|| {
            init_dbt_pool().ok();
            DBTPOOL.get()
        })
        .ok_or(ErrorKind::DatabasePool)?;

    pool.get_conn().map_err(|_| ErrorKind::DatabaseConnection)
}

pub fn dbt_url() -> String {
    format!(
        "mysql://{user}:{password}@{server}:{port}/{database}",
        server = config::DBT_SERVER,
        port = config::DBT_PORT,
        database = config::DBT_DATABASE,
        user = config::DBT_USER,
        password = config::DBT_PASSWORD,
    )
}
