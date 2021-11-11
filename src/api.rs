use rocket_sync_db_pools::database;
#[database("rusty_crab_cookpad")]
pub struct MyDatabase(diesel::pg::PgConnection);

pub mod category;
pub mod recipe;
pub mod tag;
pub mod user;
