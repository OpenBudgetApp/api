use std::env;

use dotenvy::dotenv;
use rocket::figment::{
    util::map,
    value::{Map, Value},
};

use oba_api::api::{account, bucket, fill, transaction};
use oba_api::DbConnection;

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
        .attach(account::stage())
        .attach(transaction::stage())
        .attach(bucket::stage())
        .attach(fill::stage())
        .launch()
        .await?;
    Ok(())
}
