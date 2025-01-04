use cptserver;
use cptserver::error::Error;

mod common;

#[test]
fn dbt_test() -> Result<(), Error> {
    let _ = &mut common::get_dbt_conn()?;
    Ok(())
}
