use cptserver;
use cptserver::error::Result;

mod common;

#[test]
fn dbt_test() -> Result<()> {
    let _ = &mut common::get_dbt_conn()?;
    Ok(())
}
