use chrono::Local;
use oba_api::models::{Account, AccountForm, Bucket, BucketForm};
use rocket::local::blocking::Client;

use oba_api::api::{account, bucket, fill, transaction};
use oba_api::DbConnection;

pub struct Setup {
    pub client: Client,
}

impl Setup {
    pub fn new() -> Self {
        let client = Client::tracked(
            rocket::build()
                .attach(DbConnection::fairing())
                .attach(account::stage())
                .attach(transaction::stage())
                .attach(bucket::stage())
                .attach(fill::stage()),
        )
        .unwrap();
        client.delete(URL_TRANSACTION).dispatch().status();
        client.delete(URL_FILL).dispatch().status();
        client.delete(URL_BUCKET).dispatch().status();
        client.delete(URL_ACCOUNT).dispatch().status();
        Self { client }
    }

    #[allow(dead_code)]
    pub fn create_account(&self) -> i32 {
        self.client
            .post(URL_ACCOUNT)
            .json(&AccountForm::new(format!(
                "account_{}",
                Local::now().to_rfc3339()
            )))
            .dispatch()
            .into_json::<Account>()
            .unwrap()
            .id()
    }

    #[allow(dead_code)]
    pub fn create_bucket(&self) -> i32 {
        self.client
            .post(URL_BUCKET)
            .json(&BucketForm::new(format!(
                "bucket_{}",
                Local::now().to_rfc3339()
            )))
            .dispatch()
            .into_json::<Bucket>()
            .unwrap()
            .id()
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        self.client.delete(URL_TRANSACTION).dispatch();
        self.client.delete(URL_FILL).dispatch();
        self.client.delete(URL_BUCKET).dispatch();
        self.client.delete(URL_ACCOUNT).dispatch();
    }
}

pub const URL_TRANSACTION: &str = "/transaction";
pub const URL_ACCOUNT: &str = "/account";
pub const URL_BUCKET: &str = "/bucket";
pub const URL_FILL: &str = "/fill";
#[allow(dead_code)]
pub const TRANSACTION_NUMBER: usize = 3;
#[allow(dead_code)]
pub const ACCOUNT_NUMBER: usize = 3;
#[allow(dead_code)]
pub const BUCKET_NUMBER: usize = 3;
#[allow(dead_code)]
pub const FILL_NUMBER: usize = 3;
