use oba_api::api::{account, bucket, transaction};
use oba_api::DbConnection;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(DbConnection::fairing())
        .attach(account::stage())
        .attach(transaction::stage())
        .attach(bucket::stage())
        .launch()
        .await?;
    Ok(())
}
