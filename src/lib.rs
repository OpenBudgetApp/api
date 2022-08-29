mod schema;
pub mod models;
pub mod api;

#[macro_use]
extern crate diesel;
use rocket_sync_db_pools::database;

#[database("sqlite")]
pub struct DbConnection(diesel::SqliteConnection);
