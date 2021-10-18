use rocket_sync_db_pools::database;
#[database("rusty_crab_cookpad")]
pub struct MyDatabase(diesel::pg::PgConnection);

pub mod user;
