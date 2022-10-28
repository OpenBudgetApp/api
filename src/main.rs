use std::env;

use dotenvy::dotenv;
use rocket::{
    fairing::AdHoc,
    figment::{
        util::map,
        value::{Map, Value},
    },
    Build, Rocket,
};
#[macro_use]
extern crate diesel_migrations;

use oba_api::api::{account, bucket, fill, transaction};
use oba_api::DbConnection;

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!("./migrations");
    let db = DbConnection::get_one(&rocket)
        .await
        .expect("database connection");
    db.run(|conn| embedded_migrations::run(conn))
        .await
        .expect("diesel migrations");
    rocket
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Configure database from .env
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: Map<_, Value> = map! {
        "url" => database_url.into()
    };
    let figment = rocket::Config::figment().merge(("databases", map!["sqlite" => db]));

    let _rocket = rocket::custom(figment)
        .attach(DbConnection::fairing())
        .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
        .attach(account::stage())
        .attach(transaction::stage())
        .attach(bucket::stage())
        .attach(fill::stage())
        .launch()
        .await?;
    Ok(())
}
